import { API_URL } from "@/scripts/api";
import * as ApiTypes from "@/scripts/apiTypes";

export async function getAllUsers(
	token: string,
): Promise<ApiTypes.UserView[]> {
	let r = await $fetch<ApiTypes.UserView[]>(API_URL + "/admin/users", {
		method: "GET",
		headers: {
			"Content-Type": "application/json",
			Authorization: `Bearer ${token}`,
		},
	});
	return r;
}

export async function deleteRecipe(
	token: string,
	id: string,
): Promise<void> {
	await $fetch(API_URL + `/admin/recipe/${id}`, {
		method: "DELETE",
		headers: {
			"Content-Type": "application/json",
			Authorization: `Bearer ${token}`,
		},
	});
}
