
library(tidyverse)
library(raster)

setwd('D:/PopDyn_nextcloud/Projects/asf_bavaria_model_kuerschner_t/Prototype/asf_model_bav_prototype')

grid <- read.csv("./output/all_grid_states.csv", header = T)

individuals <- read.csv("./output/all_individuals.csv", header = T)



g1 <- grid %>% filter(iteration == 1)

i1 <- individuals %>% filter(iteration == 1)

g2 <- g1 %>% mutate(quality = case_when(quality == -9999 ~ 0))

(p1<-ggplot(g2)+
  geom_tile(aes(x=x, y=y, fill = quality))+
  scale_fill_gradient(low = "#d1dde0", high = "#313232") +
  geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 5, position=position_dodge(width=1))
)


(p2<-ggplot(individuals)+
  geom_line(aes(iteration,age, color= as.factor(id)))
)


r1 <- raster("./input/landscape/redDeer_global_50m.asc")

plot(r1)


nrow(r1)


