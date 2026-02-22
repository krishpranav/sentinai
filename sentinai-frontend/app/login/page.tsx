"use client";

import { useState } from "react";
import { useAuth } from "../../components/authprovider";
import { Button } from "../../components/button";
import { api, ApiError } from "../../lib/api";

export default function LoginPage() {
    const { login } = useAuth();
    const [token, setTokenInput] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setLoading(true);
        setError(null);

        try {
            const response = await api.post("/auth/github", { access_token: token });
            login(response.token, response.user);

            window.location.href = "/dashboard";
        } catch (err: any) {
            if (err instanceof ApiError) {
                setError(err.message);
            } else {
                setError("Failed to authenticate. Please try again.");
            }
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="flex flex-1 items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
            <div className="card w-full max-w-md space-y-8 p-10">
                <div>
                    <h2 className="mt-2 text-center text-3xl font-bold tracking-tight text-white">
                        Sign in to Sentinai
                    </h2>
                    <p className="mt-2 text-center text-sm text-gray-400">
                        Use <span className="text-gray-300 font-mono">mock-token</span> for local development.
                    </p>
                </div>
                <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
                    {error && (
                        <div className="p-3 rounded-md bg-red-900/50 border border-red-800 text-sm text-red-200">
                            {error}
                        </div>
                    )}
                    <div>
                        <label htmlFor="token" className="block text-sm font-medium text-gray-300">
                            Personal Access Token
                        </label>
                        <div className="mt-1">
                            <input
                                id="token"
                                name="token"
                                type="password"
                                required
                                value={token}
                                onChange={(e) => setTokenInput(e.target.value)}
                                className="block w-full rounded-md border-gray-700 bg-gray-800 px-3 py-2 text-white placeholder-gray-500 focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                                placeholder="ghp_***********"
                            />
                        </div>
                    </div>

                    <Button
                        type="submit"
                        className="w-full"
                        disabled={loading || !token}
                        size="lg"
                    >
                        {loading ? "Authenticating..." : "Sign in via GitHub"}
                    </Button>
                </form>
            </div>
        </div>
    );
}
