"use client";

import { useEffect, useState } from "react";
import ProtectedRoute from "../../components/protectedroute";
import Sidebar from "../../components/sidebar";
import { api } from "../../lib/api";
import { Project, SecurityFinding } from "../../lib/types";

export default function DashboardPage() {
    const [projects, setProjects] = useState<Project[]>([]);
    const [findings, setFindings] = useState<SecurityFinding[]>([]);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        async function fetchData() {
            try {
                const [projsData, secData] = await Promise.all([
                    api.get("/projects"),
                    api.get("/security")
                ]);

                setProjects(projsData);
            } catch (err) {
                console.error(err);
            } finally {
                setIsLoading(false);
            }
        }

        fetchData();
    }, []);

    return (
        <ProtectedRoute>
            <div className="flex flex-1">
                <Sidebar />
                <main className="flex-1 p-8 overflow-auto">
                    <div className="max-w-7xl mx-auto space-y-6">
                        <h1 className="text-2xl font-bold tracking-tight text-white mb-8">
                            System Overview
                        </h1>

                        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                            <div className="card">
                                <h3 className="text-sm font-medium text-gray-400">Total Projects</h3>
                                <p className="text-4xl font-semibold text-white mt-2">
                                    {isLoading ? "-" : projects.length}
                                </p>
                            </div>
                            <div className="card">
                                <h3 className="text-sm font-medium text-gray-400">Monitored Pipelines</h3>
                                <p className="text-4xl font-semibold text-white mt-2">
                                    {isLoading ? "-" : projects.length}
                                </p>
                            </div>
                            <div className="card">
                                <h3 className="text-sm font-medium text-gray-400">Active Vulnerabilities</h3>
                                <p className="text-4xl font-semibold text-red-500 mt-2">
                                    {isLoading ? "-" : findings.length}
                                </p>
                            </div>
                        </div>

                        <div className="card mt-8">
                            <h2 className="text-lg font-medium text-white mb-4">Recent Activity</h2>
                            <p className="text-sm text-gray-400">
                                {projects.length === 0
                                    ? "No activity recorded. Start by creating a project."
                                    : "All systems operating normally. Monitoring active CI/CD processes."}
                            </p>
                        </div>
                    </div>
                </main>
            </div>
        </ProtectedRoute>
    );
}
