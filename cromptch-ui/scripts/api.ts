import * as ApiTypes from "@/scripts/apiTypes";

const API_URL = "http://127.0.0.1:3001/api";

export function register(
	username: string,
	email: string,
	password: string,
	hcaptchaToken?: string,
) {
	let body: Record<string, string> = { username, email, password };
	if (hcaptchaToken) {
		body["hcaptchaToken"] = hcaptchaToken;
	}
	return $fetch(API_URL + "/user/create", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(body),
	});
}

export function getImageUrl(id: string): string {
	return `${API_URL}/image/${id}`;
}

export function getImageThumbnailUrl(id: string): string {
	return `${API_URL}/image/thumbnail/${id}`;
}

export async function login(
	email: string,
	password: string,
): Promise<ApiTypes.LoginResponse> {
	let r = await $fetch<ApiTypes.LoginResponse>(API_URL + "/user/login", {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify({ email, password }),
	});
	return r;
}

export async function getSelfUser(
	token: string,
): Promise<ApiTypes.GetSelfResponse> {
	let r = await $fetch<ApiTypes.GetSelfResponse>(API_URL + "/user/self", {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
			Authorization: `Bearer ${token}`,
		},
	});
	return r;
}

export async function getRecipeList(
	limit: number = 10,
): Promise<ApiTypes.RecipeMetadata[]> {
	let r = await $fetch<ApiTypes.RecipeMetadata[]>(
		`${API_URL}/recipe/list?limit=${limit}`,
		{
			method: "GET",
			headers: {
				"Content-Type": "application/json",
			},
		},
	);
	return r;
}

export async function getRecipe(id: string): Promise<ApiTypes.Recipe> {
	let r = await $fetch<ApiTypes.Recipe>(`${API_URL}/recipe/${id}`, {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
		},
	});
	return r;
}

export async function createRecipe(
	body: ApiTypes.CreateRecipeRequest,
	token: string,
): Promise<ApiTypes.CreateRecipeResponse> {
	let r = await $fetch<ApiTypes.CreateRecipeResponse>(
		`${API_URL}/recipe/create`,
		{
			method: "POST",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${token}`,
			},
			body,
		},
	);
	return r;
}

export async function uploadImage(
	imageFile: Blob,
	token: string,
): Promise<ApiTypes.ImageUploadResponse> {
	let body = new FormData();
	body.set("file", imageFile);
	let r = await $fetch<ApiTypes.ImageUploadResponse>(
		`${API_URL}/image`,
		{
			method: "POST",
			headers: {
				Authorization: `Bearer ${token}`,
			},
			body,
		},
	);
	return r;
}
