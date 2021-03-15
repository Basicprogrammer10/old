tellraw @a ["",{"text":"Minecraft "},{"text":"Spawn","underlined":true},{"text":" Datapack "},{"text":"V0.01","color":"aqua"},{"text":" By "},{"text":"[Connor Slade]","color":"green","bold":true,"underlined":false,"clickEvent":{"action":"open_url","value":"https://github.com/Basicprogrammer10/Spawn-Datapack"}}]

gamerule commandBlockOutput false

scoreboard objectives add spawn trigger
scoreboard objectives add cancel trigger

scoreboard players enable @a spawn
scoreboard players enable @a cancel

scoreboard objectives add cooldown dummy
scoreboard objectives add to_spawn dummy

execute as @e[type=minecraft:armor_stand, tag=spawn] run kill @s
summon armor_stand ~ ~ ~ {Invisible:1,NoGravity:1b,Tags:["spawn"]}