execute as @a run execute if score @s spawn matches 1 if score @s cooldown matches 0 run schedule function spawn:do_tp 3s
execute as @a run execute if score @s spawn matches 1 if score @s cooldown matches 0 run tellraw @a ["",{"text":"[*]","color":"light_purple"},{"text":" You will teleport in 3 seconds.","color":"green"},{"text":" "},{"text":"[CANCEL]","color":"red","clickEvent":{"action":"run_command","value":"/trigger cancel"}}]
execute as @a run execute if score @s spawn matches 1 if score @s cooldown matches 1 run tellraw @a ["",{"text":"[*]","color":"light_purple"},{"text":" Slow Down!","color":"aqua"}]
execute as @a run execute if score @s spawn matches 1 if score @s cooldown matches 0 run scoreboard players set @s cooldown 1
execute as @a run execute if score @s spawn matches 1 if score @s cooldown matches 1 run scoreboard players set @s to_spawn 1
execute as @a run execute if score @s spawn matches 1 if score @s cooldown matches 1 run scoreboard players set @s spawn 0

execute as @a run execute if score @s cancel matches 1 run function spawn:cancel