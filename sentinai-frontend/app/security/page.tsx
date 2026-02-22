"use client";

import { useEffect, useState } from "react";
import ProtectedRoute from "../../components/protectedroute";
import Sidebar from "../../components/sidebar";
import { api } from "../../lib/api";
import { Project, SecurityFinding } from "../../lib/types";

export default function SecurityPage() {
    const [findings, setFindings] = useState<SecurityFinding[]>([]);
    const [projects, setProjects] = useState<Project[]>([]);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        async function loadSecurityData() {
            try {
                const projs: Project[] = await api.get("/projects");
                setProjects(projs);

                let allFindings: SecurityFinding[] = [];

                for (const proj of projs) {
                    try {
                        const f: SecurityFinding[] = await api.get(`/projects/${proj.id}/security`);
                        allFindings = [...allFindings, ...f];
                    } catch (e) {
                        console.warn(`Failed to load security for project ${proj.id}`);
                    }
                }

                setFindings(allFindings.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()));
            } catch (err) {
                console.error(err);
            } finally {
                setIsLoading(false);
            }
        }

        loadSecurityData();
    }, []);

    const getSeverityBadge = (severity: string) => {
        switch (severity) {
            case "critical": return "bg-red-900/50 text-red-400 border border-red-800";
            case "high": return "bg-orange-900/50 text-orange-400 border border-orange-800";
            case "medium": return "bg-yellow-900/50 text-yellow-400 border border-yellow-800";
            default: return "bg-blue-900/50 text-blue-400 border border-blue-800";
        }
    };

    return (
        <ProtectedRoute>
            <div className="flex flex-1">
                <Sidebar />
                <main className="flex-1 p-8 overflow-auto">
                    <div className="max-w-7xl mx-auto">
                        <h1 className="text-2xl font-bold tracking-tight text-white mb-8">
                            Global Security Posture
                        </h1>

                        {isLoading ? (
                            <p className="text-gray-400">Loading security data...</p>
                        ) : findings.length === 0 ? (
                            <div className="card text-center py-12">
                                <p className="text-gray-400">No vulnerabilities detected across your projects.</p>
                            </div>
                        ) : (
                            <div className="card overflow-hidden !p-0">
                                <table className="w-full text-sm text-left">
                                    <thead className="text-xs text-gray-400 uppercase bg-gray-900 border-b border-gray-800">
                                        <tr>
                                            <th className="px-6 py-4 font-medium">Severity</th>
                                            <th className="px-6 py-4 font-medium">Description</th>
                                            <th className="px-6 py-4 font-medium">Project</th>
                                            <th className="px-6 py-4 font-medium">Status</th>
                                            <th className="px-6 py-4 font-medium text-right">Detected</th>
                                        </tr>
                                    </thead>
                                    <tbody className="divide-y divide-gray-800">
                                        {findings.map((finding) => {
                                            const proj = projects.find(p => p.id === finding.project_id);
                                            return (
                                                <tr key={finding.id} className="hover:bg-gray-800/50 transition-colors bg-gray-900">
                                                    <td className="px-6 py-4">
                                                        <span className={`px-2 py-1 rounded text-xs font-semibold uppercase ${getSeverityBadge(finding.severity)}`}>
                                                            {finding.severity}
                                                        </span>
                                                    </td>
                                                    <td className="px-6 py-4 text-gray-200">
                                                        {finding.description}
                                                    </td>
                                                    <td className="px-6 py-4 text-gray-400">
                                                        {proj?.name || "Unknown"}
                                                    </td>
                                                    <td className="px-6 py-4">
                                                        {finding.resolved ? (
                                                            <span className="text-green-500 font-medium tracking-tight">Resolved</span>
                                                        ) : (
                                                            <span className="text-red-400 font-medium tracking-tight">Action Required</span>
                                                        )}
                                                    </td>
                                                    <td className="px-6 py-4 text-gray-500 text-right">
                                                        {new Date(finding.created_at).toLocaleDateString()}
                                                    </td>
                                                </tr>
                                            )
                                        })}
                                    </tbody>
                                </table>
                            </div>
                        )}
                    </div>
                </main>
            </div>
        </ProtectedRoute>
    );
}
