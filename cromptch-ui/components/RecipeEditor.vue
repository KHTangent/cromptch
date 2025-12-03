<template>
	<v-container>
		<v-form ref="recipeEditor" validate-on="submit" @submit.prevent="submit">
			<h2 class="text-h2">Metadata</h2>
			<v-text-field
				v-model="recipe.name"
				label="Recipe name"
				required
				class="ma-3"
				:rules="[(v) => !!v || 'Title is required']"
			></v-text-field>
			<v-textarea
				v-model="recipe.description"
				label="Description"
				class="ma-3"
			></v-textarea>
			<v-divider class="my-2"></v-divider>
			<h2 class="text-h2 my-4">Optional info</h2>
			<v-btn block @click="mainImageDialogOpen = true"> Add image </v-btn>
			<ImageUploadDialog
				title="Main image"
				v-model:open="mainImageDialogOpen"
				v-model:uploadedUuid="recipe.imageId"
			/>
			<v-row class="mt-2">
				<v-col md="6" cols="12">
					<v-text-field
						v-model="recipe.timeEstimateActive"
						type="number"
						label="Estimated time spent preparing (hours)"
						required
						:rules="[
							(v) =>
								!v ||
								v.length == 0 ||
								v > 0 ||
								'Amount must be empty or greater than 0',
						]"
					></v-text-field>
				</v-col>
				<v-col md="6" cols="12">
					<v-text-field
						v-model="recipe.timeEstimateTotal"
						type="number"
						label="Estimated time to done, including waiting (hours)"
						required
						:rules="[
							(v) =>
								!v ||
								v.length == 0 ||
								v > 0 ||
								'Amount must be empty or greater than 0',
						]"
					></v-text-field>
				</v-col>
				<v-col cols="12">
					<v-text-field
						v-model="recipe.sourceUrl"
						label="Recipe source URL"
						required
						:rules="[
							(v) =>
								!v ||
								v.length == 0 ||
								v.startsWith('http://') ||
								v.startsWith('https://') ||
								'Must be empty or a valid URL',
						]"
					></v-text-field>
				</v-col>
			</v-row>
			<v-divider class="my-2"></v-divider>
			<h2 class="text-h2 my-4">Ingredients</h2>
			<v-row>
				<v-col cols="12">
					<v-list>
						<v-list-item v-for="(ingredient, i) in recipe.ingredients" :key="i">
							<v-row>
								<v-col cols="2">
									<v-text-field
										v-model="ingredient.quantity"
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
										v-model="ingredient.unit"
										:items="unitSuggestions"
										auto-select-first
										label="Unit"
										required
										:rules="[(v) => !!v || 'Unit is required']"
									></v-autocomplete>
								</v-col>
								<v-col cols="8">
									<v-text-field
										v-model="ingredient.name"
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
					<div v-for="(step, i) in recipe.steps" :key="i">
						<v-textarea
							v-model="step.description"
							rows="1"
							auto-grow
							:label="'Step ' + (i + 1)"
							:rules="[(v) => !!v || 'Step is required']"
							required
							class="ma-3"
						>
							<template #prepend>
								<v-btn @click="openStepImageUploadDialog(i)" icon>
									<v-icon>{{ icons.mdiCamera }}</v-icon>
								</v-btn>
								<ImageUploadDialog
									:title="`Step ${i + 1} image`"
									v-model:open="recipeDialogsOpen[i]"
									v-model:uploadedUuid="step.imageId"
								/>
							</template>
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
			<v-btn type="submit" block size="large" color="success" class="ma-3">
				Submit
			</v-btn>
		</v-form>
	</v-container>
</template>
<script setup lang="ts">
import { type CreateRecipeRequest } from "@/scripts/apiTypes";
import { mdiClose, mdiPlus, mdiCamera } from "@mdi/js";

const recipe = defineModel<CreateRecipeRequest>({ required: true });
const emit = defineEmits(["submit"]);

const recipeEditor = ref<any>(null);
const mainImageDialogOpen = ref(false);
const recipeDialogsOpen = ref<Array<boolean>>(
	new Array(recipe.value.steps.length).fill(false),
);

const icons = { mdiClose, mdiPlus, mdiCamera };
const unitSuggestions = [
	"g",
	"kg",
	"ml",
	"dl",
	"l",
	"tsp",
	"tbsp",
	"cup",
	"pinch",
	"piece",
	"pack",
	"bag",
	"box",
	"can",
];

async function submit() {
	const { valid } = await recipeEditor.value.validate();
	if (!valid) {
		return;
	}
	emit("submit");
}

function addIngredient(after: number) {
	recipe.value.ingredients.splice(after + 1, 0, {
		quantity: 0.0,
		unit: "",
		name: "",
	});
}

function removeIngredient(index: number) {
	recipe.value.ingredients.splice(index, 1);
	if (recipe.value.ingredients.length === 0) {
		recipe.value.ingredients.push({
			quantity: 0.0,
			unit: "",
			name: "",
		});
	}
}

function addStep(after: number) {
	recipeDialogsOpen.value.push(false);
	recipe.value.steps.splice(after + 1, 0, {
		description: "",
		imageId: "",
	});
}

function openStepImageUploadDialog(index: number) {
	recipeDialogsOpen.value[index] = true;
}

function removeStep(index: number) {
	recipe.value.steps.splice(index, 1);
	recipeDialogsOpen.value.pop();
	if (recipe.value.steps.length === 0) {
		recipeDialogsOpen.value.push(false);
		recipe.value.steps.push({
			description: "",
			imageId: "",
		});
	}
}
</script>
