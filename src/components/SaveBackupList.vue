<script setup lang="ts">
import { NCard, NButton, NList, NListItem, NPagination, NSpace, NModal, NInput, NPopconfirm, NCheckbox } from "naive-ui";
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
const selectedBackupIds = ref<string[]>([]);

const editModalVisible = ref(false);
const editingNote = ref('');
const editingBackupId = ref<string | null>(null);
const editLoading = ref(false);

const currentPage = ref(1);
const pageSize = ref(5);

const backupDisabled = computed(() => !props.selectedGame);

const restoreDisabled = computed(() => selectedBackupIds.value.length !== 1);

const deleteDisabled = computed(() => selectedBackupIds.value.length === 0);

const isAllSelected = computed(() => {
  return paginatedList.value.length > 0 && paginatedList.value.every(item => selectedBackupIds.value.includes(item.id));
});

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
      name: '',
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
  if (selectedBackupIds.value.length !== 1) {
    props.showMessage?.('warning', '请先选择一个备份');
    return;
  }
  
  backupLoading.value = true;
  try {
    await invoke<string>("restore_save", {
      backupId: selectedBackupIds.value[0]
    });
    
    props.showMessage?.('success', '恢复成功');
  } catch (error) {
    console.error("Restore failed:", error);
    props.showMessage?.('error', `恢复失败: ${error}`);
  } finally {
    backupLoading.value = false;
  }
}

function toggleSelect(backupId: string) {
  const index = selectedBackupIds.value.indexOf(backupId);
  if (index === -1) {
    selectedBackupIds.value.push(backupId);
  } else {
    selectedBackupIds.value.splice(index, 1);
  }
}

function toggleSelectAll() {
  if (isAllSelected.value) {
    const paginatedIds = paginatedList.value.map(item => item.id);
    selectedBackupIds.value = selectedBackupIds.value.filter(id => !paginatedIds.includes(id));
  } else {
    const paginatedIds = paginatedList.value.map(item => item.id);
    const newIds = paginatedIds.filter(id => !selectedBackupIds.value.includes(id));
    selectedBackupIds.value = [...selectedBackupIds.value, ...newIds];
  }
}

function openEditModal(item: DisplayBackupItem) {
  editingBackupId.value = item.id;
  editingNote.value = item.note;
  editModalVisible.value = true;
}

async function handleUpdateNote() {
  if (!editingBackupId.value) return;
  
  editLoading.value = true;
  try {
    await invoke("update_backup_note", {
      backupId: editingBackupId.value,
      note: editingNote.value
    });
    
    const index = backupList.value.findIndex(b => b.id === editingBackupId.value);
    if (index !== -1) {
      backupList.value[index].note = editingNote.value;
    }
    
    props.showMessage?.('success', '备注更新成功');
    editModalVisible.value = false;
  } catch (error) {
    console.error("Failed to update note:", error);
    props.showMessage?.('error', `备注更新失败: ${error}`);
  } finally {
    editLoading.value = false;
  }
}

function handleEditKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    handleUpdateNote();
  }
}

async function handleDelete() {
  if (selectedBackupIds.value.length === 0) {
    props.showMessage?.('warning', '请先选择至少一个备份');
    return;
  }
  
  backupLoading.value = true;
  try {
    if (selectedBackupIds.value.length === 1) {
      await invoke("delete_backup", {
        backupId: selectedBackupIds.value[0]
      });
    } else {
      await invoke("delete_backups", {
        backupIds: selectedBackupIds.value
      });
    }
    
    props.showMessage?.('success', '删除成功');
    selectedBackupIds.value = [];
    await loadBackupList();
  } catch (error) {
    console.error("Delete failed:", error);
    props.showMessage?.('error', `删除失败: ${error}`);
  } finally {
    backupLoading.value = false;
  }
}
</script>

<template>
  <NCard title="存档备份列表" bordered>
    <div class="card-content">
      <div v-if="backupList.length > 0" class="list-header">
        <NCheckbox :checked="isAllSelected" @update:checked="toggleSelectAll">全选</NCheckbox>
        <span class="selected-count" v-if="selectedBackupIds.length > 0">已选择 {{ selectedBackupIds.length }} 项</span>
      </div>
      <NList v-if="backupList.length > 0" hoverable clickable>
        <NListItem 
          v-for="item in paginatedList" 
          :key="item.id"
          :class="{ 'backup-item-selected': selectedBackupIds.includes(item.id) }"
          @click="toggleSelect(item.id)"
        >
          <div class="backup-item-content">
            <NCheckbox 
              :checked="selectedBackupIds.includes(item.id)" 
              @update:checked="toggleSelect(item.id)"
              @click.stop
            />
            <div class="backup-item-info">
              <span class="backup-name">{{ item.note || '无备注' }}</span>
              <span class="backup-date">{{ item.date }}</span>
            </div>
            <div class="backup-actions">
              <NButton text type="primary" @click.stop="openEditModal(item)">✏️</NButton>
              <span class="backup-size">{{ item.size }}</span>
            </div>
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
          <NPopconfirm @positive-click="handleDelete">
            <template #trigger>
              <NButton type="error" :disabled="deleteDisabled" :loading="backupLoading">删除</NButton>
            </template>
            确定要删除该备份吗？此操作不可撤销。
          </NPopconfirm>
        </NSpace>
      </div>
    </template>
  </NCard>

  <NModal v-model:show="editModalVisible" preset="dialog" title="修改备注" :loading="editLoading" @positive-click="handleUpdateNote">
    <NInput v-model:value="editingNote" placeholder="请输入备注" @keydown="handleEditKeydown" />
  </NModal>
</template>

<style scoped>
.card-content {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border-bottom: 1px solid #eee;
}

.selected-count {
  font-size: 12px;
  color: #666;
}

.card-content :deep(.n-list) {
  flex: 1;
  overflow: auto;
}

.backup-item-content {
  display: flex;
  align-items: center;
  gap: 12px;
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

.backup-actions {
  display: flex;
  align-items: center;
  gap: 8px;
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
