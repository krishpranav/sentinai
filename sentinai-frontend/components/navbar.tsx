"use client";

import Link from "next/link";
import { useAuth } from "./authprovider";
import { Button } from "./button";

export default function Navbar() {
    const { user, logout } = useAuth();

    return (
        <nav className="border-b border-gray-800 bg-gray-950 h-16 flex items-center px-6 sticky top-0 z-50">
            <div className="flex items-center justify-between w-full max-w-7xl mx-auto">
                <Link href="/" className="flex items-center gap-2">
                    <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
                        <span className="text-white font-bold text-lg">S</span>
                    </div>
                    <span className="text-xl font-semibold tracking-tight text-white hidden sm:block">
                        Sentinai
                    </span>
                </Link>

                <div className="flex items-center gap-4">
                    {user ? (
                        <>
                            <span className="text-sm text-gray-400 hidden md:block">
                                {user.email || user.username}
                            </span>
                            <Button variant="ghost" size="sm" onClick={logout}>
                                Log out
                            </Button>
                        </>
                    ) : (
                        <Link href="/login">
                            <Button variant="primary" size="sm">
                                Log in
                            </Button>
                        </Link>
                    )}
                </div>
            </div>
        </nav>
    );
}
