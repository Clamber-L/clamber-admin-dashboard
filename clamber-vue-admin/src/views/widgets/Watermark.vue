<template>
    <div class="page-content">
        <!-- 基础文字水印 -->
        <el-card class="card" shadow="never">
            <template #header>基础文字水印</template>
            <el-watermark :content="name" :font="{ color: 'rgba(128, 128, 128, 0.2)' }">
                <div style="height: 200px"></div>
            </el-watermark>
        </el-card>

        <!-- 多行文字水印 -->
        <el-card class="card" shadow="never">
            <template #header>多行文字水印</template>
            <el-watermark
                :content="[name, '专注用户体验，视觉设计']"
                :font="{ fontSize: 16, color: 'rgba(128, 128, 128, 0.2)' }">
                <div style="height: 200px"></div>
            </el-watermark>
        </el-card>

        <!-- 图片水印 -->
        <el-card class="card" shadow="never">
            <template #header>图片水印</template>
            <el-watermark :image="watermarkImage" :opacity="0.2" :width="80" :height="20">
                <div style="height: 200px"></div>
            </el-watermark>
        </el-card>

        <!-- 自定义样式水印 -->
        <el-card class="card" shadow="never">
            <template #header>自定义样式水印</template>
            <el-watermark
                :content="name"
                :font="{
                    fontSize: 20,
                    fontFamily: 'Arial',
                    color: 'rgba(255, 0, 0, 0.3)'
                }"
                :rotate="-22"
                :gap="[100, 100]">
                <div style="height: 200px"></div>
            </el-watermark>
        </el-card>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useSettingStore } from '@/store/modules/setting'
import AppConfig from '@/config'

const settingStore = useSettingStore()
const { name } = AppConfig.systemInfo

// 这里替换成你的实际logo图片地址
const watermarkImage = ref('https://element-plus.org/images/element-plus-logo.svg')

const handleWatermarkVisible = () => {
    useSettingStore().setWatermarkVisible(!settingStore.watermarkVisible)
}
</script>

<style lang="scss" scoped>
.page-content {
    padding: 20px;

    .el-card {
        margin-bottom: 30px;
    }
}
</style>
