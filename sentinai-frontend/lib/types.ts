export interface User {
  id: string;
  github_id: number;
  username: string;
  email: string | null;
  created_at: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface Project {
  id: string;
  user_id: string;
  name: string;
  repository_url: string;
  created_at: string;
}

export interface Pipeline {
  id: string;
  project_id: string;
  yaml_config: string;
  created_at: string;
}

export interface SecurityFinding {
  id: string;
  project_id: string;
  severity: "low" | "medium" | "high" | "critical";
  description: string;
  resolved: boolean;
  created_at: string;
}
