import Link from "next/link";
import { Button } from "../components/button";

export default function LandingPage() {
    return (
        <div className="flex-1 flex flex-col items-center justify-center text-center px-6">
            <h1 className="text-5xl md:text-7xl font-bold tracking-tight text-white mb-6">
                Secure CI/CD <br />
                <span className="text-blue-500">at the speed of thought.</span>
            </h1>
            <p className="max-w-2xl text-lg text-gray-400 mb-10">
                Sentinai orchestrates advanced security findings, intelligent pipeline generations,
                and institutional grade project management in one unified cloud platform.
            </p>
            <div className="flex items-center gap-4">
                <Link href="/login">
                    <Button size="lg" className="px-8 font-semibold">
                        Get Started
                    </Button>
                </Link>
                <Link href="/login">
                    <Button size="lg" variant="secondary" className="px-8 font-semibold">
                        Login
                    </Button>
                </Link>
            </div>
        </div>
    );
}
