<template>
	<v-container>
		<h1 class="text-h1 mb-4">Cromptch Admin</h1>

		<h2 class="text-h2 my-4">Users</h2>
		<v-data-table :items="userList"></v-data-table>

		<v-divider class="my-8"></v-divider>

		<h2 class="text-h2 my-4">Recipes</h2>
		<v-data-table :items="recipeList">
			<template v-slot:item.actions="{ item }">
				<v-btn icon @click="deleteItem(item.id)">
					<v-icon>
						{{ icons.mdiTrashCan }}
					</v-icon>
				</v-btn>
			</template>
		</v-data-table>
	</v-container>
</template>
<script setup lang="ts">
import * as API from "@/scripts/api";
import * as ApiTypes from "@/scripts/apiTypes";
import * as AdminAPI from "@/scripts/adminApi";
import { mdiTrashCan } from "@mdi/js";

const icons = { mdiTrashCan };

const token = useToken();

const userList = ref(await AdminAPI.getAllUsers(token.value));
var recipeList = ref((await API.getRecipeList(ApiTypes.RecipeListSortTypes.NameAscending, 99999)).map((r: any) => {
	r["actions"] = "";
	return r;
}));

async function deleteItem(id: string) {
	await AdminAPI.deleteRecipe(token.value, id);
	recipeList.value = (await API.getRecipeList(ApiTypes.RecipeListSortTypes.NameAscending, 99999)).map((r: any) => {
		r["actions"] = "";
		return r;
	});
}
</script>
