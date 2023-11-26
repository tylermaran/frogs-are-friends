# That asteroid keeps getting bigger

![image](https://github.com/tylermaran/terminus/assets/30934424/622ed9ca-81ee-4e65-be2e-5f87150a5fd5)
(general UI idea)

## Libraries

- https://github.com/tokio-rs/axum
- https://github.com/ratatui-org/ratatui

## Key mechanics

- Civilization builder in space
- Multiplayer
- All terminal / text based
- Websockets, so live events

## Gameplay

- Start off on a colony ship
- Select a starting planet
- Planets will have a number of fields on them (8, 16, 32, 64, 128, etc.)
- Multiple players can inhabit one planet
- Different fields will have different properties (i.e. water, ice, rock, forest)
- Different planets will have different properties (icey, gas giant, proximity to the sun, etc.)
- The "ship ai" will select a good starting planet / location for you. (i.e. matching your species / game level / resource needs). You will be able to yolo it and choose your own location if you wish.
- There will be different species. Some will be human like (i.e. prefer earth planets) others will live on icy methane worlds, or within gas giants. This will help protect new players from players trying to colonize their planets. I.e. if a human want to colonize a icy planet, they will need a higher technology level to overcome the cold / food production.

## Resource Gathering

- I want to have slightly more resource options than ogame (which had metal, crystal, deuterium). I think there should at least be: [iron, titanium, uranium, lithium, silicone, hydrocarbons, methane, etc.]. With each planet having comparatively higher / lower quantities of each.
- Thoughts on technical side of resource management: https://github.com/tylermaran/frogs-are-friends/issues/1

## Species

I like the idea of having multiple different species to play as. Definitely going to focus on humans for the first launch, but want to have a competing terraforming aspect at some point. 

- Humans
  - Normal humans. Need agriculture, water, and 255 - 310k temperature ranges.
- Siku
  - Live on the icy planets. Eat methane or some other hydrocarbon.
- Mercs
  - Whatever kind of species that would enjoy Mercury
- Titans
  - Live within gas giants

## Terraforming planets

- Each species should have some unique ability to terraform a planet to their liking. i.e. mercs want to burn off the atmosphere and increase the temperature to their liking. Titans should have some way of vaporizing the planets surface and increasing gravity.
- Any cross species terraforming would be disasterous to the other species on the planet.

## Trading resources

- Should be possible, but no idea yet

## Combat

- Gonna take a lot of work. Ideally there is ground and space combat. You can fight for control of a planet, or to defend a planet from an interstellar attack. Probably going to figure out all the resource management stuff before we get into this.

## Planets

- Hexagons: https://www.redblobgames.com/grids/hexagons/
- Sizing:
- Each planet will have a number of fields. Thinking this will be powers of 2. So 2, 4, 6, 8, 16, 32, 64, 128, 256, 512.
- An earthlike planet will be 64 fields. And a mars will be 32 fields (since earth is roughly 2x the size of mars). An earth size moon would be 8 (1/8th the size of earth)
- The 256 / 512s will be reserved for the gas giants.
- The 2-8 range will be moons and asteroids

Thinking that a planet overview will look something like the below: 
- Planet with 32 fields. Each field will support some amount of resource production / population.
- You can expand into adjacent fields with ground vehicles, or skip over fields with some air transporation.
- The planet wraps. This is a Robinson projection. You should be able to move from Field 1 to Field 8, likewise you could  move top to bottom (i.e. over the poles)
![image](https://github.com/tylermaran/frogs-are-friends/assets/30934424/71823004-ae1e-4285-9f07-5fe06f47dc33)


## Ship concept
![image](https://github.com/tylermaran/terminus/assets/30934424/e51f9bd3-27b4-41df-ae17-cd3ef054d01f)
