<script setup lang="ts">
import { NCard, NButton, NInput, NForm, NFormItem } from "naive-ui";
</script>

<template>
  <NCard title="存档备份列表" bordered>
    <div class="card-content">
      <p>这是右侧卡片的内容区域，可以放置任何组件或文本。</p>
      <NForm>
        <NFormItem label="输入框">
          <NInput placeholder="请输入内容" />
        </NFormItem>
        <NButton type="primary">右侧按钮</NButton>
      </NForm>
    </div>
  </NCard>
</template>

<style scoped>
.card-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
  height: 100%;
}
</style>
