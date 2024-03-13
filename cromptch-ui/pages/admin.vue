<template>
	<v-container>
		<h1 class="text-h1 mb-4">Cromptch Admin</h1>
		<h2 class="text-h2 my-4">Users</h2>
		<v-data-table :items="userList"></v-data-table>
	</v-container>
</template>
<script setup lang="ts">
import * as API from "@/scripts/api";
import * as AdminAPI from "@/scripts/adminApi";
import * as APITypes from "@/scripts/apiTypes";

const token = useToken();
const user = await useLocalUser();

const userList = await AdminAPI.getAllUsers(token.value);
</script>
