<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref } from 'vue';
import router from '../router';

type XZMU = {
    account: {
        username: String,
        password: String,
    }
    config: {
        wlan_user_ip: String,
        wlan_user_mac: String,
        wlan_ac_ip: String,
        wlan_ac_name: String,
    };
};

const show_success = ref(false);
const save_account = ref(false);
const show_error = ref(false);
const error_message = ref('');

const go_login = () => {
    router.push('/login');
}

onMounted(async () => {


    invoke('init_app').then((res) => {
        const xzmu = res as XZMU;
        console.log(xzmu);
        let account = xzmu.account;
        let config = xzmu.config;

        if (account != null) {
            invoke('login', { account: account, config: config }).then((res) => {
                console.log(res);
                // [Log] dr1003({"result":0,"msg":"ldap auth error","ret_code":1}); (Home.vue, line 24)
                // [Log] dr1003({"result":1,"msg":"Portal协议认证成功！"}); (Home.vue, line 24)
                let s = res as string;
                if (s.includes("Portal")) {
                    show_success.value = true;
                } else if (s.includes("ldap auth error")) {
                    show_error.value = true;
                    error_message.value = s;
                }

            }).catch((err) => {
                console.log(err);
            });
        } else {
            go_login();
        }


    }).catch((err) => {
        console.log(err);
        show_success.value = true;
    })


})
</script>
<template>

    <div v-if="show_success">
        <v-alert class="mt-3 ml-3 mr-3 mb-3" text="欢迎使用西藏民族大学校园网络" title="您已登陆" type="success"></v-alert>
        <v-alert v-if="save_account" class="mt-3 ml-3 mr-3 mb-3" text="检测到您未保存凭证！" title="未保存凭证" type="info"
            variant="tonal"></v-alert>

    </div>

    <div v-if="show_error">
        <v-alert class="mt-3 ml-3 mr-3 mb-3" :text="error_message" title="错误" type="error" variant="tonal"></v-alert>
    </div>

    <div class="mt-3 ml-3 mr-3 mb-3">
        <v-btn rounded="lg" size="x-large" @click="go_login()" block>重置凭证</v-btn>
    </div>
</template>


<style scoped></style>