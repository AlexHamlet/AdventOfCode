class example:


    def main():
        print("Running Main")

#Can be run withou an example object
def greeting(name):
    print(f"Hello, {name}")

#Runs only if run directly
if __name__ == "__main__":
    example.main()