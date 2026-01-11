import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { decrypt, importKeyFromBase64 } from '../lib/crypto'
import { secretsApi, type SecretResponse } from '../lib/api'

export default function ViewSecret() {
  const { token } = useParams<{ token: string }>()
  const navigate = useNavigate()
  const [secret, setSecret] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [password, setPassword] = useState('')
  const [needsPassword, setNeedsPassword] = useState(false)

  useEffect(() => {
    if (!token) {
      setError('Invalid token')
      setLoading(false)
      return
    }

    loadSecret()
  }, [token])

  const loadSecret = async () => {
    try {
      // Get key from URL hash
      const keyBase64 = window.location.hash.substring(1)
      
      if (!keyBase64) {
        setNeedsPassword(true)
        setLoading(false)
        return
      }

      // Fetch encrypted secret
      const encryptedSecret: SecretResponse = await secretsApi.get(token!)

      // Import key
      const key = await importKeyFromBase64(keyBase64)

      // Decrypt
      const encryptedData = JSON.parse(encryptedSecret.encrypted_data)
      const decrypted = await decrypt(encryptedData, key)

      setSecret(decrypted)
      
      // If burn after reading, delete immediately
      if (encryptedSecret.burn_after_reading) {
        await secretsApi.delete(token!)
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load secret')
    } finally {
      setLoading(false)
    }
  }

  const handlePasswordSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    // TODO: Implement password-based decryption
    setError('Password-based decryption not yet implemented')
  }

  if (loading) {
    return (
      <div className="px-4 py-8 sm:px-6 lg:px-8">
        <div className="max-w-2xl mx-auto text-center">
          <p className="text-gray-600">Loading secret...</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="px-4 py-8 sm:px-6 lg:px-8">
        <div className="max-w-2xl mx-auto">
          <div className="bg-red-50 border border-red-200 rounded-lg p-6">
            <h2 className="text-xl font-bold text-red-900 mb-2">Error</h2>
            <p className="text-red-700">{error}</p>
            <button
              onClick={() => navigate('/')}
              className="mt-4 text-red-600 hover:text-red-800"
            >
              Go Home
            </button>
          </div>
        </div>
      </div>
    )
  }

  if (needsPassword) {
    return (
      <div className="px-4 py-8 sm:px-6 lg:px-8">
        <div className="max-w-2xl mx-auto">
          <div className="bg-white shadow rounded-lg p-6">
            <h2 className="text-2xl font-bold text-gray-900 mb-4">Enter Password</h2>
            <form onSubmit={handlePasswordSubmit}>
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                placeholder="Password"
                className="w-full px-3 py-2 border border-gray-300 rounded-md mb-4"
              />
              <button
                type="submit"
                className="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700"
              >
                Decrypt
              </button>
            </form>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="px-4 py-8 sm:px-6 lg:px-8">
      <div className="max-w-2xl mx-auto">
        <div className="bg-white shadow rounded-lg p-6">
          <h2 className="text-2xl font-bold text-gray-900 mb-4">Secret</h2>
          <div className="bg-gray-50 rounded-md p-4">
            <pre className="whitespace-pre-wrap text-sm">{secret}</pre>
          </div>
          <button
            onClick={() => navigate('/')}
            className="mt-4 text-indigo-600 hover:text-indigo-800"
          >
            Create New Secret
          </button>
        </div>
      </div>
    </div>
  )
}

