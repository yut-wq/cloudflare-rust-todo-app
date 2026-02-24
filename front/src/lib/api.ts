import { dev } from '$app/environment';
import { env } from '$env/dynamic/public';

export interface Todo {
	id: number;
	text: string;
	completed: boolean;
}

export interface CreateTodoRequest {
	text: string;
}

export interface UpdateTodoRequest {
	completed: boolean;
}

// API基本URL（環境変数から動的に取得、フォールバック付き）
const getApiBaseUrl = (): string => {
	// 1. PUBLIC_API_BASE_URL 環境変数から取得
	if (env.PUBLIC_API_BASE_URL) {
		return env.PUBLIC_API_BASE_URL;
	}

	// 2. デフォルト値（開発/本番環境で分岐）
	return dev
		? 'http://localhost:8787' // 開発時のワーカーURL
		: 'https://my-rust-worker.your-subdomain.workers.dev'; // 本番のワーカーURL
};

const API_BASE_URL = getApiBaseUrl();

// デバッグ用：現在使用されているAPIのURLをログ出力（開発環境のみ）
if (dev) {
	console.log('API Base URL:', API_BASE_URL);
	console.log('Environment variables:', {
		PUBLIC_API_BASE_URL: env.PUBLIC_API_BASE_URL,
		dev
	});
}

class TodoApi {
	private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
		const url = `${API_BASE_URL}${endpoint}`;

		const response = await fetch(url, {
			headers: {
				'Content-Type': 'application/json',
				...options?.headers
			},
			...options
		});

		if (dev) {
			console.log(`API Request: ${url}`, {
				method: options?.method || 'GET',
				status: response.status
			});
		}

		if (!response.ok) {
			throw new Error(`API request failed: ${response.status} ${response.statusText}`);
		}

		// 204 No Contentの場合は空のレスポンスを返す
		if (response.status === 204) {
			return {} as T;
		}

		return response.json();
	}

	// 全TODO取得
	async getTodos(): Promise<Todo[]> {
		return this.request<Todo[]>('/api/todos');
	}

	// TODO作成
	async createTodo(text: string): Promise<Todo> {
		return this.request<Todo>('/api/todos', {
			method: 'POST',
			body: JSON.stringify({ text })
		});
	}

	// TODO更新（完了状態の切り替え）
	async updateTodo(id: number, completed: boolean): Promise<Todo> {
		return this.request<Todo>(`/api/todos/${id}`, {
			method: 'PUT',
			body: JSON.stringify({ completed })
		});
	}

	// TODO削除
	async deleteTodo(id: number): Promise<void> {
		await this.request<void>(`/api/todos/${id}`, {
			method: 'DELETE'
		});
	}
}

export const todoApi = new TodoApi();
