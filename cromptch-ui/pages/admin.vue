<template>
	<v-container>
		<h1 :class="`text-h${isMobile ? '2' : '1'} mb-4`">Cromptch Admin</h1>

		<h2 class="text-h2 my-4">Users</h2>
		<v-data-table :items="userList"></v-data-table>

		<v-divider class="my-8"></v-divider>

		<h2 class="text-h2 my-4">Recipes</h2>
		<ConfirmationDialog
			:title="!deleteRecipeConfirmationOpen ? '' : `Delete recipe ${recipeList[selectedRecipe].id}?`"
			v-model="deleteRecipeConfirmationOpen"
			@confirm="deleteRecipe(recipeList[selectedRecipe].id)"
		>
			<p class="text-body-1">
				This action can not be undone.
			</p>
		</ConfirmationDialog>
		<v-data-table :items="recipeList">
			<template v-slot:item.actions="{ index }">
				<v-btn icon @click="selectedRecipe = index; deleteRecipeConfirmationOpen = true">
					<v-icon>
						{{ icons.mdiTrashCan }}
					</v-icon>
				</v-btn>
			</template>
		</v-data-table>
	</v-container>
</template>
<script setup lang="ts">
import * as ApiTypes from "@/scripts/apiTypes";
import { mdiTrashCan } from "@mdi/js";

const icons = { mdiTrashCan };

const token = useToken();
const isMobile = useDisplay().mobile;

const userList = ref(await useBackend().admin.getAllUsers(token.value));

interface RecipeMetadataWithActions extends ApiTypes.RecipeMetadata {
	actions: string;
}

function extendRecipe(recipe: ApiTypes.RecipeMetadata) {
	const r = recipe as RecipeMetadataWithActions;
	r.actions = "";
	return r;
}
const recipeList = ref((await useBackend().getRecipeList(ApiTypes.RecipeListSortTypes.NameAscending, 99999)).map(extendRecipe));
const selectedRecipe = ref(-1);
const deleteRecipeConfirmationOpen = ref(false);

async function deleteRecipe(id: string) {
	await useBackend().admin.deleteRecipe(token.value, id);
	recipeList.value = (await useBackend().getRecipeList(ApiTypes.RecipeListSortTypes.NameAscending, 99999)).map(extendRecipe);
	deleteRecipeConfirmationOpen.value = false;
}
</script>
