print("CREATING LUA CONTEXT!")

crafter = {
    blocks = {
    }
}

print("LUA CONTEXT CREATED")

print("INITIALIZING DEBUG FUNCTIONS")

crafter.register_block = function(name, texture)
    crafter.blocks[name] = {
        name = name,
        texture = texture
    }
end

print("NED INITIALIZING DEBUG FUNCTIONS")

print("TESTING DEBUG FUNCTIONS")

crafter.register_block("dirt", "dirt.png")

crafter.register_block("debug_alpha", "debug_alpha.png")


print("ENDING TESTING DEBUG FUNCTIONS")