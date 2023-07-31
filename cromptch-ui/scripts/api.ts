import * as ApiTypes from "@/scripts/apiTypes";

const API_URL = "http://127.0.0.1:8000";

export function register(username: string, email: string, password: string) {
	return fetch(API_URL + "/user/create", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({ username, email, password }),
	});
}

export async function login(
	email: string,
	password: string,
): Promise<ApiTypes.LoginResponse> {
	let r = await fetch(API_URL + "/user/login", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({ email, password }),
	});
	if (r.status === 200) {
		return r.json();
	} else throw r;
}

export async function getSelfUser(
	token: string,
): Promise<ApiTypes.GetSelfResponse> {
	let r = await fetch(API_URL + "/user/self", {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
			Authorization: `Bearer ${token}`,
		},
	});
	if (r.status === 200) {
		return r.json();
	} else throw r;
}

export async function getRecipeList(
	limit: number = 10,
): Promise<ApiTypes.RecipeMetadata[]> {
	let r = await fetch(`${API_URL}/recipe/list?limit=${limit}`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});
	if (r.status === 200) {
		return r.json();
	} else throw r;
}

export async function getRecipe(id: string): Promise<ApiTypes.Recipe> {
	let r = await fetch(`${API_URL}/recipe/${id}`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});
	if (r.status === 200) {
		return r.json();
	} else throw r;
}
