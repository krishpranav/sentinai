"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import ProtectedRoute from "../../components/protectedroute";
import Sidebar from "../../components/sidebar";
import { Button } from "../../components/button";
import { api } from "../../lib/api";
import { Project } from "../../lib/types";

export default function ProjectsPage() {
    const [projects, setProjects] = useState<Project[]>([]);
    const [isLoading, setIsLoading] = useState(true);

    const [isCreating, setIsCreating] = useState(false);
    const [name, setName] = useState("");
    const [repoUrl, setRepoUrl] = useState("");

    const loadProjects = async () => {
        try {
            const data = await api.get("/projects");
            setProjects(data);
        } catch (err) {
            console.error(err);
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        loadProjects();
    }, []);

    const handleCreate = async (e: React.FormEvent) => {
        e.preventDefault();
        setIsCreating(true);
        try {
            await api.post("/projects", { name, repository_url: repoUrl });
            setName("");
            setRepoUrl("");
            await loadProjects();
        } catch (err) {
            console.error(err);
            alert("Failed to create project");
        } finally {
            setIsCreating(false);
        }
    };

    const handleDelete = async (id: string) => {
        if (!confirm("Delete this project permanently?")) return;
        try {
            await api.delete(`/projects/${id}`);
            await loadProjects();
        } catch (err) {
            console.error(err);
            alert("Failed to delete project");
        }
    };

    return (
        <ProtectedRoute>
            <div className="flex flex-1 overflow-hidden">
                <Sidebar />
                <main className="flex-1 p-8 overflow-y-auto">
                    <div className="max-w-5xl mx-auto space-y-8">
                        <div className="flex items-center justify-between">
                            <h1 className="text-2xl font-bold tracking-tight text-white">
                                Projects
                            </h1>
                        </div>

                        {/* Create Project Form */}
                        <div className="card">
                            <h2 className="text-lg font-medium text-white mb-4">Register New Repository</h2>
                            <form onSubmit={handleCreate} className="flex items-end gap-4">
                                <div className="flex-1">
                                    <label className="block text-sm font-medium text-gray-400 mb-1">Project Name</label>
                                    <input
                                        required
                                        type="text"
                                        value={name}
                                        onChange={(e) => setName(e.target.value)}
                                        className="block w-full rounded-md border-gray-700 bg-gray-800 px-3 py-2 text-white focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                                        placeholder="backend-api"
                                    />
                                </div>
                                <div className="flex-1">
                                    <label className="block text-sm font-medium text-gray-400 mb-1">Repository URL</label>
                                    <input
                                        required
                                        type="url"
                                        value={repoUrl}
                                        onChange={(e) => setRepoUrl(e.target.value)}
                                        className="block w-full rounded-md border-gray-700 bg-gray-800 px-3 py-2 text-white focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                                        placeholder="https://github.com/org/repo"
                                    />
                                </div>
                                <Button type="submit" disabled={isCreating || !name || !repoUrl}>
                                    {isCreating ? "Creating..." : "Add Project"}
                                </Button>
                            </form>
                        </div>

                        {/* Project List */}
                        <div className="space-y-4">
                            <h2 className="text-lg font-medium text-white">Active Projects</h2>
                            {isLoading ? (
                                <p className="text-gray-400">Loading projects...</p>
                            ) : projects.length === 0 ? (
                                <div className="card text-center py-12">
                                    <p className="text-gray-400">No projects registered yet.</p>
                                </div>
                            ) : (
                                <div className="grid grid-cols-1 gap-4">
                                    {projects.map((project) => (
                                        <div key={project.id} className="card flex items-center justify-between hover:border-gray-700 transition-colors">
                                            <div className="min-w-0 flex-1">
                                                <Link href={`/projects/${project.id}`} className="block focus:outline-none">
                                                    <h3 className="text-lg font-semibold text-blue-400 hover:text-blue-300 truncate">
                                                        {project.name}
                                                    </h3>
                                                    <p className="text-sm text-gray-400 truncate mt-1">
                                                        {project.repository_url}
                                                    </p>
                                                </Link>
                                            </div>
                                            <div className="flex items-center gap-4 ml-6">
                                                <span className="text-xs text-gray-500 hidden sm:block">
                                                    Created {new Date(project.created_at).toLocaleDateString()}
                                                </span>
                                                <Button
                                                    variant="danger"
                                                    size="sm"
                                                    onClick={() => handleDelete(project.id)}
                                                >
                                                    Delete
                                                </Button>
                                            </div>
                                        </div>
                                    ))}
                                </div>
                            )}
                        </div>

                    </div>
                </main>
            </div>
        </ProtectedRoute>
    );
}
