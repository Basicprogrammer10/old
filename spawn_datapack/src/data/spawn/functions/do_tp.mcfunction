execute as @a run execute if score @s to_spawn matches 1 run tellraw @s ["",{"text":"[*]","color":"light_purple"},{"text":" Welcome to Spawn ","color":"green"},{"selector":"@p","color":"blue"},{"text":"!","color":"green"}]
execute as @a run execute if score @s to_spawn matches 1 run execute at @e[type=minecraft:armor_stand, tag=spawn] run teleport @s ~ ~ ~
scoreboard players set @a to_spawn 0
scoreboard players enable @a spawn
schedule clear spawn:do_tp
schedule function spawn:enable_commands 5s