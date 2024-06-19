import * as ApiTypes from "@/scripts/apiTypes";
import { AdminApi } from "./adminApi";

export const API_URL = "";

export class Api {
	protected apiUrl: string;
	admin: AdminApi;

	constructor(apiUrl: string) {
		this.apiUrl = apiUrl;
		this.admin = new AdminApi(apiUrl);
	}

	register(
		username: string,
		email: string,
		password: string,
		hcaptchaToken?: string,
	) {
		let body: Record<string, string> = { username, email, password };
		if (hcaptchaToken) {
			body["hcaptchaToken"] = hcaptchaToken;
		}
		return $fetch(this.apiUrl + "/user/create", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(body),
		});
	}

	getImageUrl(id: string): string {
		return `${this.apiUrl}/image/${id}`;
	}

	getImageThumbnailUrl(id: string): string {
		return `${this.apiUrl}/image/thumbnail/${id}`;
	}

	async login(
		email: string,
		password: string,
	): Promise<ApiTypes.LoginResponse> {
		let r = await $fetch<ApiTypes.LoginResponse>(this.apiUrl + "/user/login", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ email, password }),
		});
		return r;
	}

	async getSelfUser(
		token: string,
	): Promise<ApiTypes.UserView> {
		let r = await $fetch<ApiTypes.UserView>(this.apiUrl + "/user/self", {
			method: "GET",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${token}`,
			},
		});
		return r;
	}

	async getRecipeList(
		sortBy: ApiTypes.RecipeListSortTypes,
		limit: number = 10,
	): Promise<ApiTypes.RecipeMetadata[]> {
		let r = await $fetch<ApiTypes.RecipeMetadata[]>(
			`${this.apiUrl}/recipe/list?limit=${limit}&order=${sortBy}`,
			{
				method: "GET",
				headers: {
					"Content-Type": "application/json",
				},
			},
		);
		return r;
	}

	async getRecipe(id: string): Promise<ApiTypes.GetRecipeResponse> {
		let r = await $fetch<ApiTypes.GetRecipeResponse>(`${this.apiUrl}/recipe/${id}`, {
			method: "GET",
			headers: {
				"Content-Type": "application/json",
			},
		});
		return r;
	}

	async createRecipe(
		body: ApiTypes.CreateRecipeRequest,
		token: string,
	): Promise<ApiTypes.CreateRecipeResponse> {
		let r = await $fetch<ApiTypes.CreateRecipeResponse>(
			`${this.apiUrl}/recipe/create`,
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

	async uploadImage(
		imageFile: Blob,
		token: string,
	): Promise<ApiTypes.ImageUploadResponse> {
		let body = new FormData();
		body.set("file", imageFile);
		let r = await $fetch<ApiTypes.ImageUploadResponse>(
			`${this.apiUrl}/image`,
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
}
