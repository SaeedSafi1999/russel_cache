using CacheUI.Models.Shared;
using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Text;
using System.Threading.Tasks;

namespace CacheUI.Services.ClusterServices
{
    public class ClusterService
    {

        public async Task<List<string>> GetClusters()
        {
            HttpClient client = new HttpClient();
            var response = await client.GetAsync("http://localhost:5022/api/get_clusters");
            var responseBody = await response.Content.ReadAsStringAsync();
            var res = JsonConvert.DeserializeObject<ApiResponse<List<string>>>(responseBody);
            if(res.Data.Count() <= 0)
                return new List<string>();
            return res.Data;
        }
    }
}
