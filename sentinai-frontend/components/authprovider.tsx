"use client";

import React, { createContext, useContext, useState, useEffect } from "react";
import { User } from "../lib/types";
import { setToken, getToken, logout as globalLogout } from "../lib/auth";
import { api } from "../lib/api";

interface AuthContextType {
    user: User | null;
    isLoading: boolean;
    login: (token: string, user: User) => void;
    logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
    const [user, setUser] = useState<User | null>(null);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        const initAuth = async () => {
            const token = getToken();
            if (token && !user) {
                try {
                    const fetchedUser = await api.get("/auth/me");
                    setUser(fetchedUser);
                } catch (error) {
                    globalLogout();
                }
            }
            setIsLoading(false);
        };

        initAuth();
    }, [user]);

    const login = (token: string, newUser: User) => {
        setToken(token);
        setUser(newUser);
    };

    const handleLogout = () => {
        setUser(null);
        globalLogout();
    };

    return (
        <AuthContext.Provider value={{ user, isLoading, login, logout: handleLogout }}>
            {children}
        </AuthContext.Provider>
    );
}

export function useAuth() {
    const context = useContext(AuthContext);
    if (context === undefined) {
        throw new Error("useAuth must be used within an AuthProvider");
    }
    return context;
}
