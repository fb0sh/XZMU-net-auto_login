<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import router from '../router';

const save_account_func = () => {
    invoke('save_account', {
        username: username.value,
        password: password.value,
    }).then(() => {
        console.log("save_account success");
        router.push('/');
    }).catch((err) => {
        console.log(err)
    });
};
const username = ref('');
const password = ref('');
</script>
<template>
    <v-sheet class="mx-auto mt-10" width="300">
        <v-form fast-fail @submit.prevent>
            <v-text-field v-model="username" label="学号"></v-text-field>

            <v-text-field v-model="password" type="password" label="密码"></v-text-field>

            <v-btn class="mt-2" type="submit" @click="save_account_func()" block>保存</v-btn>
        </v-form>
    </v-sheet>
</template>


<style scoped></style>
