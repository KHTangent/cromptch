export interface LoginResponse {
	id: string;
	username: string;
	email: string;
	isAdmin: boolean;
	token: string;
}

export interface UserView {
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
	imageId?: string;
	timeEstimateActive?: number;
	timeEstimateTotal?: number;
	sourceUrl?: string;
	createdAt: number;
	editedAt: number;
}

export interface RecipeIngredient {
	quantity: number;
	unit: string;
	name: string;
}

export interface RecipeStep {
	description: string;
	imageId?: string;
}

export interface Recipe {
	metadata: RecipeMetadata;
	ingredients: Array<RecipeIngredient>;
	steps: Array<RecipeStep>;
}

export interface CreateRecipeRequest {
	name: string;
	description: string;
	imageId?: string;
	timeEstimateActive?: number;
	timeEstimateTotal?: number;
	sourceUrl?: string;
	ingredients: Array<RecipeIngredient>;
	steps: Array<RecipeStep>;
}

export interface CreateRecipeResponse {
	id: string;
}

export interface GetRecipeResponse {
	recipe: Recipe;
	author: string;
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
