let memoryToken: string | null = null;

export const setToken = (token: string | null) => {
    memoryToken = token;
};

export const getToken = () => memoryToken;

export const logout = () => {
    memoryToken = null;
    if (typeof window !== "undefined") {
        window.location.href = "/login";
    }
};
