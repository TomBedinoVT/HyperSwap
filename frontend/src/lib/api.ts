import axios from 'axios'

const API_BASE_URL = import.meta.env.VITE_API_URL || '/api'

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
})

// Add auth token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('auth_token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// Handle auth errors
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('auth_token')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

export interface CreateSecretRequest {
  encrypted_data: string
  encrypted_metadata?: string
  max_views?: number
  expires_in_days?: number
  burn_after_reading: boolean
  organization_id?: string
}

export interface SecretResponse {
  id: string
  token: string
  encrypted_data: string
  encrypted_metadata?: string
  max_views?: number
  current_views: number
  expires_at?: string
  burn_after_reading: boolean
  is_file: boolean
  file_size?: number
  file_mime_type?: string
  created_at: string
}

export const secretsApi = {
  create: async (data: CreateSecretRequest): Promise<SecretResponse> => {
    const response = await api.post<SecretResponse>('/secrets', data)
    return response.data
  },

  get: async (token: string): Promise<SecretResponse> => {
    const response = await api.get<SecretResponse>(`/secrets/${token}`)
    return response.data
  },

  delete: async (token: string): Promise<void> => {
    await api.delete(`/secrets/${token}`)
  },

  list: async (): Promise<SecretResponse[]> => {
    const response = await api.get<SecretResponse[]>('/secrets')
    return response.data
  },
}

export default api

