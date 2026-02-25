<script setup lang="ts">
import { NCard, NButton, NList, NListItem, NPagination, NSpace } from "naive-ui";
import { ref, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface GameItem {
  id: string;
  name: string;
  savePath: string;
}

interface BackupItem {
  id: string;
  note: string;
  save_time: string;
  source_path: string;
  size: string;
}

interface DisplayBackupItem {
  id: string;
  name: string;
  date: string;
  size: string;
  note: string;
  source_path: string;
}

const props = defineProps<{
  selectedGame: GameItem | null;
  showMessage?: (type: 'success' | 'error' | 'warning' | 'info', content: string) => void;
}>();

const backupLoading = ref(false);
const backupList = ref<DisplayBackupItem[]>([]);
const selectedBackupId = ref<string | null>(null);

const currentPage = ref(1);
const pageSize = ref(5);

const backupDisabled = computed(() => !props.selectedGame);

const restoreDisabled = computed(() => !selectedBackupId.value);

const paginatedList = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  const end = start + pageSize.value;
  return backupList.value.slice(start, end);
});

function formatTimestamp(timestamp: string): string {
  const num = parseInt(timestamp);
  if (isNaN(num)) return timestamp;
  
  const date = new Date(num);
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');
  
  return `${year}-${month}-${day} ${hours}:${minutes}`;
}

async function loadBackupList() {
  backupLoading.value = true;
  try {
    const list = await invoke<BackupItem[]>("get_backup_list");
    
    backupList.value = list.map(item => ({
      id: item.id,
      name: item.source_path ? item.source_path.split(/[/\\]/).pop() || '未知游戏' : '未知游戏',
      date: formatTimestamp(item.save_time),
      size: item.size,
      note: item.note,
      source_path: item.source_path
    }));
    
    currentPage.value = 1;
  } catch (error) {
    console.error("Failed to load backup list:", error);
    backupList.value = [];
  } finally {
    backupLoading.value = false;
  }
}

watch(() => props.selectedGame, () => {
  loadBackupList();
}, { immediate: true });

async function handleBackup() {
  if (!props.selectedGame) {
    props.showMessage?.('warning', '请先选择一个游戏');
    return;
  }
  
  backupLoading.value = true;
  try {
    await invoke<string>("backup_save", {
      sourcePath: props.selectedGame.savePath,
      note: "",
      gameName: props.selectedGame.name
    });
    
    props.showMessage?.('success', '备份成功');
    await loadBackupList();
  } catch (error) {
    console.error("Backup failed:", error);
    props.showMessage?.('error', `备份失败: ${error}`);
  } finally {
    backupLoading.value = false;
  }
}

async function handleRestore() {
  if (!selectedBackupId.value) {
    props.showMessage?.('warning', '请先选择一个备份');
    return;
  }
  
  backupLoading.value = true;
  try {
    await invoke<string>("restore_save", {
      backupId: selectedBackupId.value
    });
    
    props.showMessage?.('success', '恢复成功');
  } catch (error) {
    console.error("Restore failed:", error);
    props.showMessage?.('error', `恢复失败: ${error}`);
  } finally {
    backupLoading.value = false;
  }
}

function selectBackup(backupId: string) {
  selectedBackupId.value = selectedBackupId.value === backupId ? null : backupId;
}
</script>

<template>
  <NCard title="存档备份列表" bordered>
    <div class="card-content">
      <NList v-if="backupList.length > 0" hoverable clickable>
        <NListItem 
          v-for="item in paginatedList" 
          :key="item.id"
          :class="{ 'backup-item-selected': selectedBackupId === item.id }"
          @click="selectBackup(item.id)"
        >
          <div class="backup-item-content">
            <div class="backup-item-info">
              <span class="backup-name">{{ item.name }}</span>
              <span class="backup-date">{{ item.date }}</span>
              <span v-if="item.note" class="backup-note">{{ item.note }}</span>
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
          <NButton 
            type="primary" 
            :disabled="backupDisabled" 
            :loading="backupLoading"
            @click="handleBackup"
          >
            备份
          </NButton>
          <NButton :disabled="restoreDisabled" :loading="backupLoading" @click="handleRestore">恢复</NButton>
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

.backup-note {
  font-size: 12px;
  color: #666;
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

.backup-item-selected {
  background-color: rgba(24, 160, 88, 0.1);
  border-radius: 6px;
}
</style>
