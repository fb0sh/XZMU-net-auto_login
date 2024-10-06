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

const connected_xzmu = ref(false);
const connected_internet = ref(false);
const save_account = ref(false);

const show_error = ref(false);
const error_message = ref('');


const check_connect = async () => {
    console.log("check_connect automatically")
    try {
        // 测试是否已连接校园网WIFI
        connected_xzmu.value = await invoke('test_xzmu_connection')
        connected_internet.value = await invoke('test_internet_connection')
        console.log("XZMU " + connected_xzmu.value)
        console.log("INET " + connected_internet.value)

        if (connected_xzmu.value && !connected_internet.value) {
            // 已连接校园网WIFI，尝试登录
            let xzmu: XZMU = await invoke('init_app')
            let account = xzmu.account
            let config = xzmu.config

            if (account != null) {
                save_account.value = true;
                let res: string = await invoke("login", { account: account, config: config })
                // [Log] dr1003({"result":0,"msg":"ldap auth error","ret_code":1}); (Home.vue, line 24)
                // [Log] dr1003({"result":1,"msg":"Portal协议认证成功！"}); (Home.vue, line 24)
                if (res.includes("Portal")) {
                    connected_internet.value = true;
                } else if (res.includes("ldap auth error")) {
                    connected_internet.value = false;
                    show_error.value = true;
                    error_message.value = res;
                }
            } else {
                router.push('/login')
            }

        }
    } catch (error) {
        // 捕获任何错误并显示
        show_error.value = true;
        error_message.value = `出现错误: ${error}`;
    } finally {

        setTimeout(check_connect, 60000 * 60); // 1 分钟后再次执行
    }

}

// 页面加载后执行检测
onMounted(async () => {

    await check_connect();

});




</script>
<template>

    <div v-if="connected_xzmu && connected_internet">
        <v-alert class="mt-3 ml-3 mr-3 mb-3" text="欢迎使用西藏民族大学校园网络" title="您已登陆" type="success"></v-alert>
        <v-alert v-if="save_account" class="mt-3 ml-3 mr-3 mb-3" text="检测到您未保存凭证！" title="未保存凭证" type="info"
            variant="tonal"></v-alert>
    </div>
    <div v-else-if="connected_internet && !connected_xzmu">
        <v-alert class="mt-3 ml-3 mr-3 mb-3" text="您已连接互联网，但未接入校园网" title="未接入" type="success"></v-alert>
    </div>
    <div v-else-if="!connected_internet && connected_xzmu">
        <v-alert class="mt-3 ml-3 mr-3 mb-3" text="检测到您已与互联网断开连接，但未登录校园网，可能是认证错误" title="已连接WIFI"
            type="warning"></v-alert>
    </div>
    <div v-else>
        <v-alert class="mt-3 ml-3 mr-3 mb-3" text="检测到您已与互联网断开连接，并未连接校园WIFI" title="未连接" type="error"></v-alert>
    </div>


    <div v-if="show_error">
        <v-alert class="mt-3 ml-3 mr-3 mb-3" :text="error_message" title="错误" type="error" variant="tonal"></v-alert>
    </div>

    <div class="mt-3 ml-3 mr-3 mb-3">
        <v-btn rounded="lg" size="x-large" @click="router.push('/login')" block>重置凭证</v-btn>
    </div>
</template>


<style scoped></style>