import { defineStore } from 'pinia'

export const useToastStore = defineStore('toast', {
  state: () => ({
    toasts: [],
    _seq: 0,
  }),
  actions: {
    _add(message, type, duration = 5000) {
      const id = ++this._seq
      this.toasts.push({ id, message, type })
      setTimeout(() => this.dismiss(id), duration)
    },
    error(message)   { this._add(message, 'error') },
    success(message) { this._add(message, 'success') },
    info(message)    { this._add(message, 'info') },
    dismiss(id) {
      const idx = this.toasts.findIndex(t => t.id === id)
      if (idx !== -1) this.toasts.splice(idx, 1)
    },
  },
})
