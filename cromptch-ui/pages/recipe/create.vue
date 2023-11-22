<template>
	<v-container>
		<h1 class="text-h1">Create recipe</h1>
		<p class="text-body-1">
			A recipe must have at least one step and one ingredient.
		</p>
		<v-divider class="my-2"></v-divider>
		<v-alert type="error" v-if="error.length > 0" dismissible class="ma-2">
			{{ error }}
		</v-alert>
		<v-form
			ref="recipeCreationForm"
			validate-on="submit"
			@submit.prevent="submitAndRedirect"
		>
			<h2 class="text-h2">Metadata</h2>
			<v-text-field
				v-model="title"
				label="Recipe name"
				required
				class="ma-3"
				:rules="[(v) => !!v || 'Title is required']"
			></v-text-field>
			<v-textarea
				v-model="description"
				label="Description"
				class="ma-3"
			></v-textarea>
			<v-divider class="my-2"></v-divider>
			<h2 class="text-h2 my-4">Ingredients</h2>
			<v-row>
				<v-col cols="12">
					<v-list>
						<v-list-item v-for="(_, i) in ingredients" :key="i">
							<v-row>
								<v-col cols="2">
									<v-text-field
										v-model="ingredients[i][0]"
										type="number"
										label="Quantity"
										required
										:rules="[
											(v) => !!v || 'Quantity is required',
											(v) => v > 0 || 'Quantity must be greater than 0',
										]"
									></v-text-field>
								</v-col>
								<v-col cols="2">
									<v-autocomplete
										v-model="ingredients[i][1]"
										:items="unitSuggestions"
										auto-select-first
										label="Unit"
										required
										:rules="[(v) => !!v || 'Unit is required']"
									></v-autocomplete>
								</v-col>
								<v-col cols="8">
									<v-text-field
										v-model="ingredients[i][2]"
										label="Ingredient"
										required
										:rules="[(v) => !!v || 'Ingredient name is required']"
									>
										<template #append>
											<v-btn @click="removeIngredient(i)" icon>
												<v-icon>{{ icons.mdiClose }}</v-icon>
											</v-btn>
										</template>
									</v-text-field>
								</v-col>
							</v-row>
							<v-btn @click="addIngredient(i)" compact block>
								<v-icon>{{ icons.mdiPlus }}</v-icon>
							</v-btn>
						</v-list-item>
					</v-list>
				</v-col>
			</v-row>
			<v-divider class="my-2"></v-divider>
			<h2 class="text-h2">Steps</h2>
			<v-row class="mb-4">
				<v-col cols="12">
					<div v-for="(_, i) in steps" :key="i">
						<v-textarea
							v-model="steps[i]"
							rows="1"
							auto-grow
							:label="'Step ' + (i + 1)"
							:rules="[(v) => !!v || 'Step is required']"
							required
							class="ma-3"
						>
							<template #append>
								<v-btn @click="removeStep(i)" icon>
									<v-icon>{{ icons.mdiClose }}</v-icon>
								</v-btn>
							</template>
						</v-textarea>
						<v-btn @click="addStep(i)" compact block>
							<v-icon>{{ icons.mdiPlus }}</v-icon>
						</v-btn>
					</div>
				</v-col>
			</v-row>
			<v-divider class="my-2"></v-divider>
			<h2 class="text-h2 mt-8 mb-4">Publish</h2>
			<v-btn
				type="submit"
				block
				size="large"
				color="success"
				class="ma-3"
				:loading="uploading"
				>Submit</v-btn
			>
		</v-form>
	</v-container>
</template>
<script setup lang="ts">
import * as API from "@/scripts/api";
import { type CreateRecipeRequest, type CreateRecipeResponse } from "@/scripts/apiTypes";
import { mdiClose, mdiPlus } from "@mdi/js";

const icons = { mdiClose, mdiPlus };
const unitSuggestions = [
	"g",
	"kg",
	"ml",
	"l",
	"tsp",
	"tbsp",
	"cup",
	"pinch",
	"piece",
];

const error = ref("");
const recipeCreationForm = ref<any>(null);
const uploading = ref(false);

const title = ref("");
const description = ref("");
const steps = ref<string[]>([""]);
const ingredients = ref<any[][]>([[0, "", ""]]);

const userToken = useToken();

function addIngredient(after: number) {
	ingredients.value.splice(after + 1, 0, [0, "", ""]);
}

function removeIngredient(index: number) {
	ingredients.value.splice(index, 1);
	if (ingredients.value.length === 0) {
		ingredients.value.push([0, "", ""]);
	}
}

function addStep(after: number) {
	steps.value.splice(after + 1, 0, "");
}

function removeStep(index: number) {
	steps.value.splice(index, 1);
	if (steps.value.length === 0) {
		steps.value.push("");
	}
}

async function submitAndRedirect() {
	const { valid } = await recipeCreationForm.value.validate();
	if (!valid) {
		return;
	}
	uploading.value = true;
	const recipe: CreateRecipeRequest = {
		title: title.value,
		description: description.value,
		ingredients: ingredients.value.map((e) => [parseFloat(e[0]), e[1], e[2]]),
		steps: steps.value,
	};
	let response: CreateRecipeResponse;
	try {
		response = await API.createRecipe(recipe, userToken.value);
	} catch (e: unknown) {
		if (e instanceof Response) {
			error.value = await e.text();
		}
		uploading.value = false;
		return;
	}
	navigateTo(`/recipe/${response.id}`);
}
</script>
