import { Router } from 'itty-router'

const router = Router()

router.get('/todos', () => new Response('Todos Index!'))

router.get('/todos/:id', ({ params }) => new Response(`Todo #${params.id}`))

router.post('/todos', async request => {
	const content = await request.json()

	return new Response('Creating Todo: ' + JSON.stringify(content))
})

router.all('*', () => new Response('Not Found.', { status: 404 }))


addEventListener('fetch', event =>
	event.respondWith(router.handle(event.request))
)