using CacheUI.Models;
using CacheUI.Services.ClusterServices;
using GalaSoft.MvvmLight.Command;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.ComponentModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Input;

namespace CacheUI.ViewModels.Clusters
{
    public class ClusterViewModel : INotifyPropertyChanged
    {
        private readonly ClusterService _clusterService;
        public ObservableCollection<Models.Clusters> Clusters { get; set; }
        public ICommand LoadClusterCommand { get; set; }



        public ClusterViewModel()
        {
            _clusterService = new();
            Clusters = new();
            LoadClusterCommand = new RelayCommand(LoadClusters);
        }


        private async void LoadClusters()
        {
            var clusters = await _clusterService.GetClusters().ConfigureAwait(false);
            if (clusters.Count() <= 0)
                return;
            foreach (var item in clusters)
            {
                Clusters.Add(new Models.Clusters { Name = item});
            }
        }

        public event PropertyChangedEventHandler? PropertyChanged;
        protected void OnPropertyChanged(string propertyName)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
        }


    }
}
