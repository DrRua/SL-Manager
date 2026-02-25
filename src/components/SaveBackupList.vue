<script setup lang="ts">
import { NCard, NButton, NList, NListItem, NPagination, NSpace } from "naive-ui";
import { ref, computed } from "vue";

interface BackupItem {
  id: string;
  name: string;
  date: string;
  size: string;
}

const backupList = ref<BackupItem[]>([
  { id: "1", name: "存档备份1", date: "2024-01-15 10:30", size: "125 MB" },
  { id: "2", name: "存档备份2", date: "2024-01-14 15:20", size: "118 MB" },
  { id: "3", name: "存档备份3", date: "2024-01-13 09:45", size: "110 MB" },
  { id: "4", name: "存档备份4", date: "2024-01-12 18:00", size: "105 MB" },
  { id: "5", name: "存档备份5", date: "2024-01-11 12:30", size: "98 MB" },
  { id: "6", name: "存档备份6", date: "2024-01-10 16:45", size: "95 MB" },
  { id: "7", name: "存档备份7", date: "2024-01-09 11:15", size: "90 MB" },
  { id: "8", name: "存档备份8", date: "2024-01-08 14:00", size: "88 MB" },
  { id: "9", name: "存档备份9", date: "2024-01-07 10:20", size: "85 MB" },
  { id: "10", name: "存档备份10", date: "2024-01-06 17:30", size: "82 MB" },
  { id: "11", name: "存档备份11", date: "2024-01-05 13:45", size: "80 MB" },
  { id: "12", name: "存档备份12", date: "2024-01-04 09:00", size: "78 MB" },
]);

const currentPage = ref(1);
const pageSize = ref(5);

const paginatedList = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  const end = start + pageSize.value;
  return backupList.value.slice(start, end);
});

function handleBackup() {
  console.log("执行备份");
}

function handleRestore() {
  console.log("执行恢复");
}
</script>

<template>
  <NCard title="存档备份列表" bordered>
    <div class="card-content">
      <NList v-if="backupList.length > 0" hoverable clickable>
        <NListItem 
          v-for="item in paginatedList" 
          :key="item.id"
        >
          <div class="backup-item-content">
            <div class="backup-item-info">
              <span class="backup-name">{{ item.name }}</span>
              <span class="backup-date">{{ item.date }}</span>
            </div>
            <span class="backup-size">{{ item.size }}</span>
          </div>
        </NListItem>
      </NList>
      <p v-else class="empty-tip">暂无备份记录</p>
      
      <div class="pagination-wrapper">
        <NPagination 
          v-model:page="currentPage" 
          :page-size="pageSize" 
          :item-count="backupList.length"
          :page-sizes="[5, 10, 20]"
          show-size-picker
          @update:page-size="(size: number) => pageSize = size"
        />
      </div>
    </div>

    <template #footer>
      <div class="card-footer">
        <NSpace justify="end">
          <NButton type="primary" @click="handleBackup">备份</NButton>
          <NButton @click="handleRestore">恢复</NButton>
        </NSpace>
      </div>
    </template>
  </NCard>
</template>

<style scoped>
.card-content {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.card-content :deep(.n-list) {
  flex: 1;
  overflow: auto;
}

.backup-item-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.backup-item-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
  overflow: hidden;
}

.backup-name {
  font-weight: 500;
  font-size: 14px;
}

.backup-date {
  font-size: 12px;
  color: #999;
}

.backup-size {
  font-size: 12px;
  color: #666;
  flex-shrink: 0;
}

.empty-tip {
  color: #999;
  text-align: center;
  padding: 40px 0;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  padding-top: 16px;
  flex-shrink: 0;
}

.card-footer {
  display: flex;
  justify-content: flex-end;
}
</style>
