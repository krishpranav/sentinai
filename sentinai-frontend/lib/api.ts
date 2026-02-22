import { getToken, logout } from "./auth";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

export class ApiError extends Error {
    constructor(public status: number, message: string) {
        super(message);
        this.name = "ApiError";
    }
}

async function fetchWithInterceptor(endpoint: string, options: RequestInit = {}) {
    const token = getToken();

    const headers: Record<string, string> = {
        "Content-Type": "application/json",
        ...((options.headers as Record<string, string>) || {}),
    };

    if (token) {
        headers["Authorization"] = `Bearer ${token}`;
    }

    const response = await fetch(`${API_URL}${endpoint}`, {
        ...options,
        headers,
    });

    if (response.status === 401) {
        logout();
        throw new ApiError(401, "Unauthorized");
    }

    if (!response.ok) {
        let message = "An error occurred";
        try {
            const errorData = await response.json();
            message = errorData.error || message;
        } catch {
            message = await response.text();
        }
        throw new ApiError(response.status, message);
    }

    const text = await response.text();
    return text ? JSON.parse(text) : null;
}

export const api = {
    get: (endpoint: string) => fetchWithInterceptor(endpoint),
    post: (endpoint: string, body: any) =>
        fetchWithInterceptor(endpoint, {
            method: "POST",
            body: JSON.stringify(body),
        }),
    delete: (endpoint: string) =>
        fetchWithInterceptor(endpoint, {
            method: "DELETE",
        }),
};
