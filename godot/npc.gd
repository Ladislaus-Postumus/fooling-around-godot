extends CharacterBody3D


const SPEED = 5.0
const JUMP_VELOCITY = 4.5
@export var pause = 5
var elapsed = 0
var forward = Vector3.ZERO
var state = "paused"


func _physics_process(delta: float) -> void:
	# Add the gravity.
	if not is_on_floor():
		velocity += get_gravity() * delta

	# Handle jump.
	if Input.is_action_just_pressed("ui_accept") and is_on_floor():
		velocity.y = JUMP_VELOCITY

	# Get the input direction and handle the movement/deceleration.
	# As good practice, you should replace UI actions with custom gameplay actions.
	elapsed += delta
	print("time: ", elapsed)
	print("state: ", state)
	if elapsed < 5 and state == "moving":
		print("forward")
		forward = (transform.basis * Vector3(0, 0, -1)).normalized()
	elif elapsed >= 5 and state == "moving":
		print("stop forward")
		forward = Vector3.ZERO
		state = "paused"
		elapsed = 0
	elif elapsed < 3 and state == "paused":
		print("paused")
		forward = Vector3.ZERO
	else:
		print("start forward")
		forward = Vector3.ZERO
		state = "moving"
		rotate_y(PI/2)
		elapsed = 0

	var target_velocity = forward * SPEED 
	velocity = target_velocity

	move_and_slide()
