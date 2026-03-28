import axios from 'axios'
import { defineStore } from 'pinia'

export const useProjectStore = defineStore('project', {
  state: () => ({
    projects: [],
    loaded:   false,
  }),

  actions: {
    async load(force = false) {
      if (this.loaded && !force) return
      const { data } = await axios.get('/api/projects')
      this.projects = data.data
      this.loaded   = true
    },

    async create(payload) {
      const { data } = await axios.post('/api/projects', payload)
      this.projects.unshift(data)
      return data
    },

    async update(id, payload) {
      const { data } = await axios.patch(`/api/projects/${id}`, payload)
      const idx = this.projects.findIndex(p => p.id === id)
      if (idx !== -1) this.projects[idx] = { ...this.projects[idx], ...data }
      return data
    },

    async remove(id) {
      await axios.delete(`/api/projects/${id}`)
      this.projects = this.projects.filter(p => p.id !== id)
    },

    async rotateKey(id) {
      const { data } = await axios.post(`/api/projects/${id}/rotate-key`)
      const idx = this.projects.findIndex(p => p.id === id)
      if (idx !== -1) this.projects[idx] = { ...this.projects[idx], api_key: data.api_key }
      return data
    },
  },
})
