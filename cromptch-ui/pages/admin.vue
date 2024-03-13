<template>
	<v-container>
		<h1 class="text-h1 mb-4">Cromptch Admin</h1>

		<h2 class="text-h2 my-4">Users</h2>
		<v-data-table :items="userList"></v-data-table>

		<v-divider class="my-8"></v-divider>

		<h2 class="text-h2 my-4">Recipes</h2>
		<v-data-table :items="recipeList"></v-data-table>
	</v-container>
</template>
<script setup lang="ts">
import * as API from "@/scripts/api";
import * as ApiTypes from "@/scripts/apiTypes";
import * as AdminAPI from "@/scripts/adminApi";

const token = useToken();

const userList = await AdminAPI.getAllUsers(token.value);
const recipeList = await API.getRecipeList(ApiTypes.RecipeListSortTypes.NameAscending, 99999);
</script>
