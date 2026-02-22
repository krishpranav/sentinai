import type { Metadata } from "next";
import { Inter } from "next/font/google";
import { AuthProvider } from "../components/authprovider";
import Navbar from "../components/navbar";
import "../styles/globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
    title: "Sentinai | Executive Dashboard",
    description: "Advanced CI & Security Pipeline Management",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body className={inter.className}>
                <AuthProvider>
                    <div className="flex flex-col min-h-screen bg-gray-950 text-gray-100">
                        <Navbar />
                        <main className="flex-1 flex flex-col">{children}</main>
                    </div>
                </AuthProvider>
            </body>
        </html>
    );
}
