<template>
	<v-dialog max-width="640px" v-model="modalOpen" @click:outside="close">
		<v-card class="ma-4">
			<v-card-text>
				<h3 v-if="title.length > 0" class="text-h3 mb-2">{{ title }}</h3>
			</v-card-text>
			<v-container>
				<slot />
			</v-container>
			<v-container>
				<v-file-input
					show-size
					:disabled="isUploading"
					:label="fileInputLabel"
					v-model="selectedFile"
					:accept="acceptTypes"
					:error="selectionError.length > 0"
					:error-messages="selectionError"
				/>
			</v-container>
			<v-card-actions>
				<v-btn plain @click="close">Cancel</v-btn>
				<v-btn v-if="!isUploading" color="error" @click="upload">Upload</v-btn>
				<v-btn v-else color="error" disabled>Uploading...</v-btn>
			</v-card-actions>
		</v-card>
	</v-dialog>
</template>

<script setup lang="ts">
import * as Api from "@/scripts/api";
import * as ApiTypes from "@/scripts/apiTypes";
import { FetchError } from "ofetch";

const modalOpen = defineModel<boolean>("open");
const uploadedUuid = defineModel<string>("uploadedUuid");
const props = defineProps({
	title: {
		type: String,
		default: "",
	},
	maxSize: {
		type: Number,
		default: 10000000
	},
	fileInputLabel: {
		type: String,
		default: "Select file"
	},
	acceptTypes: {
		type: String,
		default: "image/png, image/jpeg, image/bmp, image/webp"
	},
});

const selectedFile = ref<File[]>([]);
const selectionError = ref<string>("");
const userToken = useToken();
const isUploading = ref(false);

function close() {
	modalOpen.value = false;
}

async function upload() {
	if (selectedFile.value.length != 1) {
		selectionError.value = "Please select a single file";
		return;
	}
	const file = selectedFile.value[0];
	if (file.size >= props.maxSize) {
		selectionError.value = `File is too large, must be max ${(props.maxSize/1000000).toFixed(1)} MB`;
		return;
	}
	if (props.acceptTypes.length > 0 && props.acceptTypes.search(file.type) == -1) {
		selectionError.value = "Invalid file type selected";
		return;
	}

	isUploading.value = true;
	let uploaded: ApiTypes.ImageUploadResponse;
	try {
		uploaded = await Api.uploadImage(file, userToken.value);
	} catch (e) {
		if (e instanceof FetchError) {
			selectionError.value = e.data;
		} else {
			selectionError.value = "Unknown error while uploading file";
		}
		isUploading.value = false;
		return;
	}

	uploadedUuid.value = uploaded.id;
	modalOpen.value = false;
	isUploading.value = false;
}
</script>
