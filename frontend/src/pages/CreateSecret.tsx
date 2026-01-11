import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { encrypt, generateKeyBase64, importKeyFromBase64 } from '../lib/crypto'
import { secretsApi } from '../lib/api'

export default function CreateSecret() {
  const [secret, setSecret] = useState('')
  const [maxViews, setMaxViews] = useState<number | undefined>()
  const [expiresInDays, setExpiresInDays] = useState<number | undefined>()
  const [burnAfterReading, setBurnAfterReading] = useState(false)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [shareUrl, setShareUrl] = useState<string | null>(null)
  const navigate = useNavigate()

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setLoading(true)
    setError(null)

    try {
      // Generate encryption key
      const keyBase64 = await generateKeyBase64()
      
      // Import key
      const key = await importKeyFromBase64(keyBase64)

      // Encrypt secret
      const encrypted = await encrypt(secret, key)

      // Create secret via API
      const response = await secretsApi.create({
        encrypted_data: JSON.stringify(encrypted),
        max_views: maxViews || undefined,
        expires_in_days: expiresInDays || undefined,
        burn_after_reading: burnAfterReading,
      })

      // Build share URL with key in hash
      const url = `${window.location.origin}/secret/${response.token}#${keyBase64}`
      setShareUrl(url)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create secret')
    } finally {
      setLoading(false)
    }
  }

  if (shareUrl) {
    return (
      <div className="px-4 py-8 sm:px-6 lg:px-8">
        <div className="max-w-2xl mx-auto">
          <div className="bg-white shadow rounded-lg p-6">
            <h2 className="text-2xl font-bold text-gray-900 mb-4">Secret Created!</h2>
            <p className="text-gray-600 mb-4">
              Your secret has been encrypted and stored. Share this link:
            </p>
            <div className="bg-gray-50 rounded-md p-4 mb-4">
              <code className="text-sm break-all">{shareUrl}</code>
            </div>
            <button
              onClick={() => {
                navigator.clipboard.writeText(shareUrl)
                alert('Link copied to clipboard!')
              }}
              className="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700"
            >
              Copy Link
            </button>
            <button
              onClick={() => navigate('/')}
              className="ml-4 text-gray-600 px-4 py-2 rounded-md hover:bg-gray-100"
            >
              Create Another
            </button>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="px-4 py-8 sm:px-6 lg:px-8">
      <div className="max-w-2xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8">Create Secret</h1>
        <form onSubmit={handleSubmit} className="bg-white shadow rounded-lg p-6">
          {error && (
            <div className="mb-4 bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
              {error}
            </div>
          )}

          <div className="mb-4">
            <label htmlFor="secret" className="block text-sm font-medium text-gray-700 mb-2">
              Secret Content
            </label>
            <textarea
              id="secret"
              value={secret}
              onChange={(e) => setSecret(e.target.value)}
              required
              rows={10}
              className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              placeholder="Enter your secret here..."
            />
          </div>

          <div className="grid grid-cols-2 gap-4 mb-4">
            <div>
              <label htmlFor="maxViews" className="block text-sm font-medium text-gray-700 mb-2">
                Max Views (optional)
              </label>
              <input
                type="number"
                id="maxViews"
                value={maxViews || ''}
                onChange={(e) => setMaxViews(e.target.value ? parseInt(e.target.value) : undefined)}
                min="1"
                className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
            <div>
              <label htmlFor="expiresInDays" className="block text-sm font-medium text-gray-700 mb-2">
                Expires In (days, optional)
              </label>
              <input
                type="number"
                id="expiresInDays"
                value={expiresInDays || ''}
                onChange={(e) => setExpiresInDays(e.target.value ? parseInt(e.target.value) : undefined)}
                min="1"
                className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
              />
            </div>
          </div>

          <div className="mb-4">
            <label className="flex items-center">
              <input
                type="checkbox"
                checked={burnAfterReading}
                onChange={(e) => setBurnAfterReading(e.target.checked)}
                className="rounded border-gray-300 text-indigo-600 focus:ring-indigo-500"
              />
              <span className="ml-2 text-sm text-gray-700">Burn after reading</span>
            </label>
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? 'Creating...' : 'Create Secret'}
          </button>
        </form>
      </div>
    </div>
  )
}

