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
	imageId?: string;
	author: string;
}

export interface Recipe {
	id: string;
	title: string;
	description: string;
	author: string;
	authorId: string;
	ingredients: any[][];
	imageId?: string;
	steps: string[];
	stepImages: Array<string | null>;
}

export interface CreateRecipeRequest {
	title: string;
	description: string;
	imageId?: string;
	ingredients: any[][];
	steps: string[];
	stepImages: Array<string | undefined>;
}

export interface CreateRecipeResponse {
	id: string;
}

export interface ImageUploadResponse {
	id: string;
}

export enum RecipeListSortTypes {
	NameAscending = "a-z",
	NameDescending = "z-a",
	DateDescending = "newest",
	DateAscending = "oldest",
}
