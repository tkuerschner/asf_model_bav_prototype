
library(tidyverse)
library(raster)

setwd('D:/PopDyn_nextcloud/Projects/asf_bavaria_model_kuerschner_t/Prototype/asf_model_bav_prototype')

grid <- read.csv("./output/all_grid_states.csv", header = T)

individuals <- read.csv("./output/all_individuals.csv", header = T)



g1 <- grid #%>% filter(iteration == 1)

i1 <- individuals %>% filter(iteration == 1)

g2 <- g1 %>% mutate(quality = case_when(quality == -9999 ~ 0))

g2 <- g1 %>% mutate(quality = replace(quality, quality<0, NA))

(p1<-ggplot(g2)+
  geom_tile(aes(x=x, y=y, fill = quality))+
  scale_fill_gradient(low = "#c2e9cf", high = "#046104") +
  geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 5, position=position_dodge(width=1))
)


cellInfo <- read.csv("./output/debugCellinfo.csv", header = T)

(ci1<-ggplot(cellInfo)+
  geom_tile(aes(x=x_grid_o, y=y_grid_o, fill = quality))+
  scale_fill_gradient(low = "#c2e9cf", high = "#046104")# +
 # geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 5, position=position_dodge(width=1))
)
#g3 <- g2 %>% mutate(y = max(y) - y)
#
#(p1<-ggplot(g3)+
#  geom_tile(aes(x=x, y=y, fill = quality))+
#  scale_fill_gradient(low = "#d1dde0", high = "#313232") +
#  geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 5, position=position_dodge(width=1))
#)
#
#g4 <- g2 %>%
#mutate(nx = y, ny = max(x)-x)
#(p1<-ggplot(g4)+
#  geom_tile(aes(x=nx, y=ny, fill = quality))+
#  scale_fill_gradient(low = "#d1dde0", high = "#313232") +
#  geom_point(data= i1,aes(x=x, y=y, color = 'red'), size = 5, position=position_dodge(width=1))
#)
#
#head(g4)

(p2<-ggplot(individuals)+
  geom_line(aes(iteration,age, color= as.factor(id)))
)


r1 <- raster("./input/landscape/redDeer_global_50m.asc")

plot(r1)



