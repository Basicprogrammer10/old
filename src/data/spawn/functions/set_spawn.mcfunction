execute align xz positioned ~0.5 ~ ~0.5 run execute as @e[type=minecraft:armor_stand, tag=spawn] run kill @s
summon armor_stand ~ ~ ~ {Invisible:1,NoGravity:1b,Tags:["spawn"]}
tellraw @a [{"text":"[*]","color":"light_purple"},{"text":" Set new Spawn Location","color":"aqua"},{"text":" Successfully!","color":"green"}]