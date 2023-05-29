# Rapier 

Region aware api 



# Q/A 


## Why rapier?
I'm a fan of the fire emblem series and the rapier is a memorable weapon from the series.
https://fireemblem.fandom.com/wiki/Rapier 


## Ok but what does this do? 
Rapier returns the region that returns the region in which a kuberenetes cluster is deployed 


## Why does this exist 
Originally intended for multicluster testing , but i ended up using something Stefan Prodans [podinfo](https://github.com/stefanprodan/podinfo)



## I see javascript where's the frontend ?
Well i suck at desgin so  that isn't ready yet.

## How do i build ? 

``` 
docker buildx build -t  rapier .
```


## How do i use ? 

using the docker image you can run the container by mounting your kubeconfig inside the container 

```
docker run -it -v /Users/username/Projects/rapier:/app -p 8080:8080 rapier

```

## Testing 

```
curl http://localhost:8080/region
```








