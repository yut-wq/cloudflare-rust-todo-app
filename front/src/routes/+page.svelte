<script lang="ts">
	import { onMount } from 'svelte';
	import { registerServiceWorker } from '$lib/pwa.js';
	import { todoApi, type Todo } from '$lib/api.js';

	let todos = $state<Todo[]>([]);
	let newTodo = $state('');
	let loading = $state(false);
	let error = $state<string | null>(null);

	// API呼び出し時のエラーハンドリング
	async function handleApiCall<T>(apiCall: () => Promise<T>): Promise<T | null> {
		try {
			loading = true;
			error = null;
			return await apiCall();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Unknown error occurred';
			console.error('API Error:', err);
			return null;
		} finally {
			loading = false;
		}
	}

	// TODOを読み込み
	async function loadTodos() {
		const result = await handleApiCall(() => todoApi.getTodos());
		if (result) {
			todos = result;
		}
	}

	// コンポーネントマウント時にTODOを読み込み
	onMount(async () => {
		await loadTodos();

		// Service Workerを登録
		registerServiceWorker();
	});

	// TODOを追加
	async function addTodo() {
		if (newTodo.trim()) {
			const result = await handleApiCall(() => todoApi.createTodo(newTodo.trim()));
			if (result) {
				todos = [...todos, result];
				newTodo = '';
			}
		}
	}

	// TODOを削除
	async function deleteTodo(id: number) {
		const result = await handleApiCall(() => todoApi.deleteTodo(id));
		if (result !== null) {
			// エラーが発生していない場合
			todos = todos.filter((todo) => todo.id !== id);
		}
	}

	// TODOの完了状態を切り替え
	async function toggleTodo(id: number) {
		const todo = todos.find((t) => t.id === id);
		if (todo) {
			const result = await handleApiCall(() => todoApi.updateTodo(id, !todo.completed));
			if (result) {
				todos = todos.map((t) => (t.id === id ? result : t));
			}
		}
	}

	// 完了済みTODOをクリア
	async function clearCompleted() {
		const completedTodos = todos.filter((todo) => todo.completed);

		// 全ての完了済みTODOを削除
		const deletionPromises = completedTodos.map((todo) =>
			handleApiCall(() => todoApi.deleteTodo(todo.id))
		);

		await Promise.all(deletionPromises);

		// UIを更新（エラーが発生していないもののみ除外）
		todos = todos.filter((todo) => !todo.completed);
	}

	// Enterキーで追加
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			addTodo();
		}
	}

	// 統計情報
	const remainingCount = $derived(todos.filter((todo) => !todo.completed).length);
	const completedCount = $derived(todos.filter((todo) => todo.completed).length);
</script>

<svelte:head>
	<title>TODO App</title>
</svelte:head>

<div class="app">
	<header>
		<h1>📝 TODO App</h1>
		<p>SvelteKit + Cloudflare Pages</p>
	</header>

	<main>
		<!-- エラー表示 -->
		{#if error}
			<div class="error">
				<p>⚠️ エラーが発生しました: {error}</p>
				<button onclick={() => (error = null)} class="error-dismiss"> 閉じる </button>
			</div>
		{/if}

		<!-- ローディング表示 -->
		{#if loading}
			<div class="loading">
				<div class="spinner"></div>
				<p>読み込み中...</p>
			</div>
		{/if}

		<!-- TODOの追加 -->
		<div class="add-todo">
			<input
				bind:value={newTodo}
				onkeydown={handleKeydown}
				placeholder="新しいTODOを入力してください..."
				class="todo-input"
				disabled={loading}
			/>
			<button onclick={addTodo} class="add-btn" disabled={!newTodo.trim() || loading}>
				追加
			</button>
		</div>

		<!-- 統計情報 -->
		{#if todos.length > 0}
			<div class="stats">
				<span class="stat">残り: {remainingCount}</span>
				<span class="stat">完了: {completedCount}</span>
				<span class="stat">合計: {todos.length}</span>
			</div>
		{/if}

		<!-- TODOリスト -->
		{#if todos.length === 0}
			<div class="empty">
				<p>TODOがありません</p>
				<p>上のフォームから新しいTODOを追加してください</p>
			</div>
		{:else}
			<div class="todo-list">
				{#each todos as todo (todo.id)}
					<div class="todo-item" class:completed={todo.completed}>
						<input
							type="checkbox"
							checked={todo.completed}
							onchange={() => toggleTodo(todo.id)}
							class="todo-checkbox"
						/>
						<span class="todo-text">{todo.text}</span>
						<button onclick={() => deleteTodo(todo.id)} class="delete-btn"> 🗑️ </button>
					</div>
				{/each}
			</div>
		{/if}

		<!-- アクション -->
		{#if completedCount > 0}
			<div class="actions">
				<button onclick={clearCompleted} class="clear-btn">
					完了済みをクリア ({completedCount})
				</button>
			</div>
		{/if}
	</main>
</div>

<style>
	/* グローバル背景とテキスト */
	:global(body) {
		background-color: #f8fafc;
		color: #1f2937;
		margin: 0;
		padding: 0;
		min-height: 100vh;
	}

	.app {
		max-width: 600px;
		margin: 0 auto;
		padding: 2rem;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		min-height: 100vh;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		color: #ffffff;
	}

	header {
		text-align: center;
		margin-bottom: 2rem;
		background: rgba(255, 255, 255, 0.1);
		padding: 2rem;
		border-radius: 16px;
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	header h1 {
		color: #ffffff;
		margin: 0;
		font-size: 2.5rem;
		text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	}

	header p {
		color: rgba(255, 255, 255, 0.8);
		margin: 0.5rem 0 0 0;
		font-size: 1rem;
	}

	.add-todo {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 2rem;
		background: rgba(255, 255, 255, 0.1);
		padding: 1.5rem;
		border-radius: 12px;
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.todo-input {
		flex: 1;
		padding: 0.75rem 1rem;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-radius: 8px;
		font-size: 1rem;
		transition: all 0.2s;
		background: rgba(255, 255, 255, 0.9);
		color: #1f2937;
	}

	.todo-input::placeholder {
		color: #6b7280;
	}

	.todo-input:focus {
		outline: none;
		border-color: #10b981;
		background: #ffffff;
		box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.1);
	}

	.add-btn {
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #10b981 0%, #059669 100%);
		color: white;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		font-size: 1rem;
		font-weight: 600;
		transition: all 0.2s;
		box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
	}

	.add-btn:hover:not(:disabled) {
		background: linear-gradient(135deg, #059669 0%, #047857 100%);
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(16, 185, 129, 0.4);
	}

	.add-btn:disabled {
		background: rgba(255, 255, 255, 0.3);
		color: rgba(255, 255, 255, 0.6);
		cursor: not-allowed;
		transform: none;
		box-shadow: none;
	}

	.error {
		background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
		color: white;
		padding: 1rem 1.5rem;
		border-radius: 12px;
		margin-bottom: 1.5rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
		box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
	}

	.error p {
		margin: 0;
		font-weight: 600;
	}

	.error-dismiss {
		background: rgba(255, 255, 255, 0.2);
		border: none;
		color: white;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 600;
		transition: background 0.2s;
	}

	.error-dismiss:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	.loading {
		text-align: center;
		padding: 2rem;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		margin-bottom: 1.5rem;
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.loading p {
		margin: 0.5rem 0 0 0;
		color: rgba(255, 255, 255, 0.8);
		font-weight: 600;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid rgba(255, 255, 255, 0.3);
		border-top: 3px solid #ffffff;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	.stats {
		display: flex;
		justify-content: center;
		gap: 2rem;
		margin-bottom: 2rem;
		padding: 1rem 1.5rem;
		background: rgba(255, 255, 255, 0.15);
		border-radius: 12px;
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.stat {
		font-weight: 600;
		color: #ffffff;
		text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
	}

	.empty {
		text-align: center;
		padding: 3rem;
		color: rgba(255, 255, 255, 0.8);
		background: rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.empty p {
		margin: 0.5rem 0;
		font-size: 1.1rem;
	}

	.todo-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 2rem;
	}

	.todo-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(255, 255, 255, 0.95);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 12px;
		transition: all 0.2s;
		backdrop-filter: blur(10px);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.todo-item:hover {
		background: rgba(255, 255, 255, 1);
		transform: translateY(-1px);
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
	}

	.todo-item.completed {
		opacity: 0.7;
		background: rgba(255, 255, 255, 0.6);
	}

	.todo-checkbox {
		width: 1.2rem;
		height: 1.2rem;
		cursor: pointer;
		accent-color: #10b981;
	}

	.todo-text {
		flex: 1;
		font-size: 1rem;
		color: #1f2937;
		transition: all 0.2s;
		font-weight: 500;
	}

	.todo-item.completed .todo-text {
		text-decoration: line-through;
		color: #6b7280;
	}

	.delete-btn {
		padding: 0.5rem;
		background: rgba(239, 68, 68, 0.1);
		border: none;
		cursor: pointer;
		font-size: 1.2rem;
		border-radius: 6px;
		transition: all 0.2s;
		color: #ef4444;
	}

	.delete-btn:hover {
		background: #ef4444;
		color: white;
		transform: scale(1.1);
	}

	.actions {
		text-align: center;
	}

	.clear-btn {
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
		color: white;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		font-size: 0.9rem;
		font-weight: 600;
		transition: all 0.2s;
		box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
	}

	.clear-btn:hover {
		background: linear-gradient(135deg, #dc2626 0%, #b91c1c 100%);
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(239, 68, 68, 0.4);
	}

	/* レスポンシブデザイン */
	@media (max-width: 640px) {
		.app {
			padding: 1rem;
		}

		header {
			padding: 1.5rem;
			margin-bottom: 1.5rem;
		}

		header h1 {
			font-size: 2rem;
		}

		.add-todo {
			flex-direction: column;
			gap: 0.75rem;
		}

		.stats {
			flex-direction: column;
			gap: 0.5rem;
			text-align: center;
		}

		.todo-item {
			padding: 0.75rem 1rem;
		}
	}

	/* ダークモード対応 */
	@media (prefers-color-scheme: dark) {
		:global(body) {
			background-color: #111827;
		}

		.app {
			background: linear-gradient(135deg, #1f2937 0%, #111827 100%);
		}
	}
</style>
