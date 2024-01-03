export interface LoginResponse {
	id: string;
	username: string;
	email: string;
	isAdmin: boolean;
	token: string;
}

export interface GetSelfResponse {
	id: string;
	username: string;
	email: string;
	isAdmin: boolean;
}

export interface RecipeMetadata {
	id: string;
	title: string;
	description: string;
	author: string;
}

export interface Recipe {
	id: string;
	title: string;
	description: string;
	author: string;
	authorId: string;
	ingredients: any[][];
	steps: string[];
}

export interface CreateRecipeRequest {
	title: string;
	description: string;
	ingredients: any[][];
	steps: string[];
}

export interface CreateRecipeResponse {
	id: string;
}

export interface ImageUploadResponse {
	id: string;
}
