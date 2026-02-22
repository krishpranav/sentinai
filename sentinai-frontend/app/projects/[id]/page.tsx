"use client";

import { useEffect, useState } from "react";
import ProtectedRoute from "../../../components/protectedroute";
import Sidebar from "../../../components/sidebar";
import { Button } from "../../../components/button";
import { api } from "../../../lib/api";
import { Project, Pipeline, SecurityFinding } from "../../../lib/types";

export default function ProjectDetailPage({ params }: { params: { id: string } }) {
    const [project, setProject] = useState<Project | null>(null);
    const [pipelines, setPipelines] = useState<Pipeline[]>([]);
    const [isGenerating, setIsGenerating] = useState(false);
    const [isLoading, setIsLoading] = useState(true);

    const loadData = async () => {
        try {
            const [projData, pipeData] = await Promise.all([
                api.get(`/projects/${params.id}`),
                api.get(`/projects/${params.id}/pipelines`)
            ]);
            setProject(projData);
            setPipelines(pipeData.sort((a: Pipeline, b: Pipeline) =>
                new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
            ));
        } catch (err) {
            console.error(err);
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        loadData();
    }, [params.id]);

    const generatePipeline = async () => {
        setIsGenerating(true);
        try {
            await api.post(`/projects/${params.id}/generate-ci`, {});
            await loadData();
        } catch (err) {
            console.error(err);
            alert("Failed to generate pipeline");
        } finally {
            setIsGenerating(false);
        }
    };

    if (isLoading) return <ProtectedRoute><div className="flex flex-1"><Sidebar /><main className="flex-1 p-8"><p className="text-gray-400">Loading project...</p></main></div></ProtectedRoute>;
    if (!project) return <ProtectedRoute><div className="flex flex-1"><Sidebar /><main className="flex-1 p-8"><p className="text-red-400">Project not found</p></main></div></ProtectedRoute>;

    return (
        <ProtectedRoute>
            <div className="flex flex-1 overflow-hidden">
                <Sidebar />
                <main className="flex-1 p-8 overflow-y-auto">
                    <div className="max-w-5xl mx-auto space-y-8">
                        <div className="flex items-center justify-between border-b border-gray-800 pb-6">
                            <div>
                                <h1 className="text-2xl font-bold tracking-tight text-white">
                                    {project.name}
                                </h1>
                                <a href={project.repository_url} target="_blank" rel="noopener noreferrer" className="text-sm text-blue-500 hover:underline mt-1 block">
                                    {project.repository_url}
                                </a>
                            </div>
                            <Button onClick={generatePipeline} disabled={isGenerating}>
                                {isGenerating ? "Generating..." : "Generate CI Pipeline"}
                            </Button>
                        </div>

                        <div className="space-y-4">
                            <h2 className="text-lg font-medium text-white">Generated Pipelines</h2>
                            {pipelines.length === 0 ? (
                                <div className="card text-center py-12">
                                    <p className="text-gray-400">No pipelines generated yet. Click the button above to analyze your repository.</p>
                                </div>
                            ) : (
                                <div className="space-y-6">
                                    {pipelines.map((pipeline) => (
                                        <div key={pipeline.id} className="card overflow-hidden !p-0">
                                            <div className="bg-gray-950 px-6 py-3 border-b border-gray-800 flex justify-between items-center">
                                                <span className="text-xs font-mono text-gray-500">ID: {pipeline.id}</span>
                                                <span className="text-xs text-gray-400">{new Date(pipeline.created_at).toLocaleString()}</span>
                                            </div>
                                            <div className="p-0 overflow-x-auto">
                                                <pre className="text-sm text-gray-300 font-mono p-6">
                                                    <code>{pipeline.yaml_config.trim()}</code>
                                                </pre>
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
