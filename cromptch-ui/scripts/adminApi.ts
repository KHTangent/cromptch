import * as ApiTypes from "@/scripts/apiTypes";

export class AdminApi {
	protected apiUrl: string;
	constructor(apiUrl: string) {
		this.apiUrl = apiUrl;
	}

	async getAllUsers(token: string): Promise<ApiTypes.UserView[]> {
		let r = await $fetch<ApiTypes.UserView[]>(this.apiUrl + "/admin/users", {
			method: "GET",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${token}`,
			},
		});
		return r;
	}

	async deleteRecipe(token: string, id: string): Promise<void> {
		await $fetch(this.apiUrl + `/admin/recipe/${id}`, {
			method: "DELETE",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${token}`,
			},
		});
	}
}
