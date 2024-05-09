<script lang="ts">
</script>

<div class=" ml-2">
	<h2 class="mb-2 text-center h2">Not yet implemented features:</h2>
	<ul class="px-4 text-left text-sm">
		<li>
			Improving performance by passing all possible moves for the turn so move
			doesn't have to be validated on the backend before updating frontend.
			<p class="text-xs">
				Also allows showing possible moves for each piece when hovered
			</p>
		</li>
		<li class="mt-2">
			Currently the backend validates the move and returns the updated chess
			object. Lag while making a move is caused by the API call to external
			backend
		</li>
		<li class="mt-2">
			Implementing a backend that can play chess against the user. This will
			probably be done by using a stockfish engine.
		</li>
		<li class="mt-2">Implementing playing other players via websocket</li>
		<li class="mt-2">Drag and drop on mobile without scrolling</li>
	</ul>
</div>
